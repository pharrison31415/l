use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use crate::primitives::{Executable, Label, MacroInvocation};

pub struct Node {
    pub elem: Executable,
    pub label: Option<Label>,
    pub prev: Option<NodePtr>,
    pub next: Option<NodePtr>,
    pub jump: Jump,
}

impl Node {
    pub fn new(elem: Executable) -> Self {
        Self {
            elem,
            label: None,
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
    pub max_label_len: usize,
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
            max_label_len: 0,
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
            label: label.clone(),
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
            self.max_label_len = self.max_label_len.max(l.0.len());
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

    pub fn get_with_label(&self) -> Option<(Option<Label>, Executable)> {
        self.pointer
            .as_ref()
            // .or(self.head.as_ref())
            .map(|p: &Rc<RefCell<Node>>| (p.borrow().label.clone(), p.borrow().elem.clone()))
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

    fn replace_node_with_list(&mut self, node: NodePtr, mut other: JumpList) {
        // Grab neighbors of the callsite node
        let (prev, next) = {
            let n = node.borrow();
            (n.prev.clone(), n.next.clone())
        };

        // If `other` is empty, we just remove `node`
        if other.head.is_none() {
            // unlink node directly (like remove_node), but without messing up due to wrong "next"
            self.size -= 1;

            match prev.as_ref() {
                Some(p) => p.borrow_mut().next = next.clone(),
                None => self.head = next.clone(),
            }
            match next.as_ref() {
                Some(n) => n.borrow_mut().prev = prev.clone(),
                None => self.tail = prev.clone(),
            }

            // (Optional) detach node fully
            {
                let mut n = node.borrow_mut();
                n.prev = None;
                n.next = None;
            }
            return;
        }

        // Non-empty replacement:
        // total size changes by (other.size - 1) because node is replaced by other's nodes
        self.size = self.size + other.size - 1;

        let other_head = other.head.take().unwrap();
        let other_tail = other.tail.take().unwrap();

        // Link prev -> other_head
        match prev.as_ref() {
            Some(p) => p.borrow_mut().next = Some(other_head.clone()),
            None => self.head = Some(other_head.clone()),
        }
        other_head.borrow_mut().prev = prev.clone();

        // Link other_tail -> next
        other_tail.borrow_mut().next = next.clone();
        match next.as_ref() {
            Some(n) => n.borrow_mut().prev = Some(other_tail.clone()),
            None => self.tail = Some(other_tail.clone()),
        }
    }

    pub fn expand_macro(&mut self, invocation: MacroInvocation, mut macro_jump_list: JumpList) {
        self.max_label_len = self.max_label_len.max(macro_jump_list.max_label_len);

        let node = self
            .unexpanded_macros
            .get(&invocation)
            .expect("Could not find call site node in jump list")
            .clone();

        // move callsite label onto the expansion head
        let callsite_label = node.borrow_mut().label.take();
        if let Some(lbl) = callsite_label {
            if let Some(ref head) = macro_jump_list.head {
                if head.borrow().label.is_some() {
                    panic!(
                        "Macro expansion label conflict: callsite label {:?} but macro head already has label {:?}",
                        lbl,
                        head.borrow().label
                    );
                }

                head.borrow_mut().label = Some(lbl.clone());

                // redirect already-resolved jumps from old callsite to new head
                self.redirect_resolved_jumps(&node, head);

                self.max_label_len = self.max_label_len.max(lbl.0.len());
                self.jump_table.insert(lbl.clone(), head.clone());

                if let Some(waiting) = self.unresolved_jumps.remove(&lbl) {
                    for w in waiting {
                        w.borrow_mut().jump = Jump::Resolved(head.clone());
                    }
                }
            } else {
                panic!(
                    "Labeled macro callsite {:?} expanded to empty list; label would have no target",
                    lbl
                );
            }
        }

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

        // Splice macro list in at the callsite position
        self.replace_node_with_list(node, macro_jump_list);
    }

    fn redirect_resolved_jumps(&mut self, from: &NodePtr, to: &NodePtr) {
        let mut cur = self.head.clone();

        while let Some(node) = cur {
            let next = node.borrow().next.clone();
            let should_redirect = match node.borrow().jump.clone() {
                Jump::Resolved(target) => Rc::ptr_eq(&target, from),
                _ => false,
            };

            if should_redirect {
                node.borrow_mut().jump = Jump::Resolved(to.clone());
            }
            cur = next;
        }
    }
}
