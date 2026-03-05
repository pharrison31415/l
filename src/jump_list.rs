use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use crate::primitives::{Executable, Label, MacroInvocation};

pub struct Node {
    pub elem: Executable,
    pub prev: Option<NodePtr>,
    pub next: Option<NodePtr>,
    pub jump: Jump,
}

impl Node {
    pub fn new(elem: Executable) -> Self {
        Self {
            elem,
            prev: None,
            next: None,
            jump: Jump::None,
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("elem", &self.elem)
            // .field("jump", &self.jump)
            .field("next", &self.next)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum Jump {
    None,
    Unresolved(Label),
    Resolved(NodePtr),
}

pub type NodePtr = Rc<RefCell<Node>>;

pub struct JumpList {
    pub file_name: String,
    pub head: Option<NodePtr>,
    pub tail: Option<NodePtr>,
    pub pointer: Option<NodePtr>,
    pub size: usize,
    pub jump_table: HashMap<Label, NodePtr>,
    pub unresolved_jumps: HashMap<Label, Vec<NodePtr>>,
    pub unexpanded_macros: HashMap<MacroInvocation, NodePtr>,
}

impl Debug for JumpList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JumpList")
            .field("head", &self.head)
            .finish()
    }
}

impl JumpList {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            head: None,
            tail: None,
            pointer: None,
            size: 0,
            jump_table: HashMap::new(),
            unresolved_jumps: HashMap::new(),
            unexpanded_macros: HashMap::new(),
        }
    }

    pub fn append(&mut self, elem: Executable, label: Option<Label>, jump_label: Option<Label>) {
        // Decide jump state
        let jump = if let Some(j) = jump_label.clone() {
            if let Some(target) = self.jump_table.get(&j) {
                Jump::Resolved(target.clone())
            } else {
                Jump::Unresolved(j)
            }
        } else {
            Jump::None
        };

        // let unexpanded_macro = matches!(elem, Executable::MacroCallSite(_));
        let mcs_opt = match elem {
            Executable::Instruction(_) => None,
            Executable::MacroCallSite(ref mcs) => Some(mcs.clone()),
        };

        let node_ptr: NodePtr = Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
            jump,
        }));

        if let Some(invocation) = mcs_opt {
            self.unexpanded_macros
                .insert(invocation.clone(), node_ptr.clone());
        }

        // Handle unresolved jump
        if let Some(j) = jump_label {
            if !self.jump_table.contains_key(&j) {
                self.unresolved_jumps
                    .entry(j)
                    .or_default()
                    .push(node_ptr.clone());
            }
        }

        // Label may resolve jumps
        if let Some(l) = label {
            self.jump_table.insert(l.clone(), node_ptr.clone());

            if let Some(waiting) = self.unresolved_jumps.remove(&l) {
                for w in waiting {
                    w.borrow_mut().jump = Jump::Resolved(node_ptr.clone());
                }
            }
        }

        // Add node to tail
        self.append_node(node_ptr);
    }

    fn append_node(&mut self, new_tail: NodePtr) {
        self.size += 1;
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn get(&self) -> Option<Executable> {
        self.pointer
            .as_ref()
            // .or(self.head.as_ref())
            .map(|p: &Rc<RefCell<Node>>| p.borrow().elem.clone())
    }

    pub fn reset_pointer(&mut self) {
        self.pointer = self.head.clone();
    }

    pub fn goto_next(&mut self) {
        let next = match self.pointer.as_ref() {
            None => None,
            Some(p) => p.borrow().next.clone(),
        };

        self.pointer = next;
    }

    pub fn goto_jump(&mut self) {
        let jump = match self.pointer.as_ref() {
            None => None,
            Some(p) => match p.borrow().jump.clone() {
                Jump::None => panic!("goto_jump called while pointing at non-jump node"),
                Jump::Unresolved(l) => panic!("goto_jump on unresolved jump: {:?}", l),
                Jump::Resolved(ref_cell) => Some(ref_cell),
            },
        };

        self.pointer = jump;
    }

    fn nest(&mut self, prev: Option<NodePtr>, other: JumpList) {
        self.size += other.size;

        let next = match prev {
            Some(ref p) => p.borrow().next.clone(),
            None => self.head.clone().unwrap().borrow().next.clone(),
        };

        match prev {
            Some(ref p) => p.borrow_mut().next = other.head.clone(),
            None => self.head = other.head.clone(),
        }
        other.head.map(|h| h.borrow_mut().prev = prev);

        other
            .tail
            .clone()
            .map(|t| t.borrow_mut().next = next.clone());
        next.map(|n| n.borrow_mut().prev = other.tail);
    }

    fn remove_node(&mut self, node: NodePtr) {
        self.size -= 1;

        let (prev, next) = {
            let mut n = node.borrow_mut();
            (n.prev.take(), n.next.take())
        };

        match prev.as_ref() {
            Some(prev_node) => prev_node.borrow_mut().next = next.clone(),
            None => self.head = next.clone(),
        }

        match next.as_ref() {
            Some(next_node) => next_node.borrow_mut().prev = prev.clone(),
            None => self.tail = prev.clone(),
        }
    }

    pub fn expand_macro(&mut self, invocation: MacroInvocation, mut macro_jump_list: JumpList) {
        let node = self
            .unexpanded_macros
            .get(&invocation)
            .expect("Could not find call site node in jump list")
            .clone();

        // Merge jump table
        for (label, target_ptr) in macro_jump_list.jump_table.drain() {
            self.jump_table.insert(label.clone(), target_ptr.clone());

            if let Some(waiting) = self.unresolved_jumps.remove(&label) {
                for w in waiting {
                    w.borrow_mut().jump = Jump::Resolved(target_ptr.clone());
                }
            }
        }

        // Merge unresolved jumps
        for (label, mut nodes) in macro_jump_list.unresolved_jumps.drain() {
            if let Some(target) = self.jump_table.get(&label).cloned() {
                for n in nodes.drain(..) {
                    n.borrow_mut().jump = Jump::Resolved(target.clone());
                }
            } else {
                self.unresolved_jumps
                    .entry(label)
                    .or_default()
                    .append(&mut nodes);
            }
        }

        let prev = node.borrow().prev.clone();

        self.nest(prev, macro_jump_list);

        self.remove_node(node);
    }
}
