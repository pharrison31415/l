use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use crate::primitives::{Executable, Label};

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
            .field("jump", &self.jump)
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
    pub head: Option<NodePtr>,
    pub tail: Option<NodePtr>,
    pub pointer: Option<NodePtr>,
    pub size: usize,
    pub jump_table: HashMap<Label, NodePtr>,
    pub unresolved_jumps: HashMap<Label, Vec<NodePtr>>,
    pub unexpanded_macros: Vec<NodePtr>,
}

impl Debug for JumpList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JumpList")
            .field("head", &self.head)
            .finish()
    }
}

impl JumpList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            pointer: None,
            size: 0,
            jump_table: HashMap::new(),
            unresolved_jumps: HashMap::new(),
            unexpanded_macros: Vec::new(),
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

        let unexpanded_macro = matches!(elem, Executable::Macro(_));

        let node_ptr: NodePtr = Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
            jump,
        }));

        if unexpanded_macro {
            self.unexpanded_macros.push(node_ptr.clone());
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
            .or(self.head.as_ref())
            .map(|p| p.borrow().elem.clone())
    }

    pub fn reset_pointer(&mut self) {
        self.pointer = self.head.clone();
    }

    pub fn goto_next(&mut self) {
        let next = match self.pointer.as_ref() {
            None => self.head.clone(),
            Some(p) => p.borrow().next.clone(),
        };

        self.pointer = next;
    }

    pub fn goto_jump(&mut self) {
        let jump = match self.pointer.as_ref() {
            None => self.head.clone(),
            Some(p) => match p.borrow().jump.clone() {
                Jump::None => panic!("goto_jump called while pointing at non-jump node"),
                Jump::Unresolved(_) => panic!("goto_jump on unresolved jump"),
                Jump::Resolved(ref_cell) => Some(ref_cell),
            },
        };

        self.pointer = jump;
    }

    fn pop_head_node(&mut self) -> Option<NodePtr> {
        self.head.take().map(|old_head| {
            old_head.borrow_mut().next.take().map(|new_head| {
                self.size += 1;
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head);
            });
            old_head
        })
    }

    fn insert_node(&mut self, prev: &NodePtr, node: NodePtr) {
        self.size += 1;
        node.borrow_mut().next = prev.borrow().next.clone();
        prev.borrow_mut().next = Some(node)
    }

    pub fn nest(&mut self, prev: NodePtr, mut other: JumpList) {
        other.reset_pointer();
        while let Some(node) = other.pop_head_node() {
            self.insert_node(&prev, node);
        }
    }
}
