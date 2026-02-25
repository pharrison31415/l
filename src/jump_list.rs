use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

type Jump<T> = Rc<RefCell<Node<T>>>;
type Link<T> = Option<Jump<T>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
    jump: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
            jump: None,
        }))
    }
}

pub struct JumpList<T, L> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
    // Map of labels to a certain jump link
    jump_table: HashMap<L, Jump<T>>,
}

impl<T, L> JumpList<T, L> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
            jump_table: HashMap::new(),
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        self.size += 1;
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    // pub fn push_back_with_label(&mut self, elem: T, label: L) {
    //     // create new_tail
    //     // add tail to hashmap with label as key
    //     // push back as normal but with new tail instead of element as arg
    // }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
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

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            self.size -= 1;
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.size -= 1;
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn into_iter(self) -> IntoIter<T, L> {
        IntoIter(self)
    }
}

impl<T, L> Drop for JumpList<T, L> {
    fn drop(&mut self) {
        self.size = 0;
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T, L>(JumpList<T, L>);

impl<T, L> Iterator for IntoIter<T, L> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T, L> DoubleEndedIterator for IntoIter<T, L> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}
