use std::cell::RefCell;
use std::rc::Rc;

pub struct SkipListNode<T: Ord + Copy> {
    pub val: T,
    pub next: Option<Rc<RefCell<SkipListNode<T>>>>,
    pub down: Option<Rc<RefCell<SkipListNode<T>>>>,
}

impl<T: Ord + Copy> SkipListNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            down: None,
        }
    }
}



