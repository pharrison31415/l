use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

pub struct Node<T, L> {
    pub elem: T,
    pub next: Option<NodePtr<T, L>>,
    pub prev: Option<NodePtr<T, L>>,
    pub jump: Jump<L, T>,
}

impl<T, L> Node<T, L> {
    pub fn new(elem: T) -> Self {
        Self {
            elem,
            next: None,
            prev: None,
            jump: Jump::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Jump<L, T> {
    None,
    Unresolved(L),
    Resolved(NodePtr<T, L>),
}

pub type NodePtr<T, L> = Rc<RefCell<Node<T, L>>>;


impl<T: Debug, L: Debug> Debug for Node<T, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("elem", &self.elem)
            .field("jump", &self.jump)
            .field("next", &self.next)
            .finish()
    }
}

pub struct JumpList<T, L> {
    pub head: Option<NodePtr<T, L>>,
    pub tail: Option<NodePtr<T, L>>,
    pub pointer: Option<NodePtr<T, L>>,
    pub size: usize,
    pub jump_table: HashMap<L, NodePtr<T, L>>,
    pub unresolved_jumps: HashMap<L, Vec<NodePtr<T, L>>>,
}

impl<T: Debug, L: Debug> Debug for JumpList<T, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JumpList")
            .field("head", &self.head)
            .finish()
    }
}

impl<T, L> JumpList<T, L>
where
    T: Debug + Clone,
    L: Debug + Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            pointer: None,
            size: 0,
            jump_table: HashMap::new(),
            unresolved_jumps: HashMap::new(),
        }
    }
    pub fn append(&mut self, elem: T, label: Option<L>, jump_label: Option<L>) {
        // Decide jump state BEFORE wrapping in Rc
        let jump = if let Some(j) = jump_label.clone() {
            if let Some(target) = self.jump_table.get(&j) {
                Jump::Resolved(target.clone())
            } else {
                Jump::Unresolved(j)
            }
        } else {
            Jump::None
        };

        let node_ptr: NodePtr<T, L> = Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
            jump,
        }));

        // If this node has an unresolved jump, remember it so it can be resolved later.
        if let Some(j) = jump_label {
            if !self.jump_table.contains_key(&j) {
                self.unresolved_jumps
                    .entry(j)
                    .or_default()
                    .push(node_ptr.clone());
            }
        }

        // If this node defines a label, store it and resolve any waiting jumps.
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

    fn append_node(&mut self, new_tail: NodePtr<T, L>) {
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

    pub fn get(&self) -> Option<T> {
        self.pointer
            .as_ref()
            .or(self.head.as_ref())
            .map(|p| p.borrow().elem.clone())
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
}
