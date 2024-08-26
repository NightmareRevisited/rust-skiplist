use std::cell::{RefCell};
use std::rc::Rc;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::skiplist_node::SkipListNode;

pub struct SkipList<T: Ord + Copy> {
    head: Option<Rc<RefCell<SkipListNode<T>>>>,
    level: usize,
    rng: ThreadRng,
}

impl<T: Ord + Copy> SkipList<T> {
    const PROMOTION_PROBABILITY: i32 = 50;

    const MAX_LEVEL: usize = 15;
    pub fn new() -> Self {
        Self {
            head: None,
            level: 1,
            rng: rand::thread_rng(),
        }
    }

    pub fn contain(&self, target: T) -> bool {
        let mut current;
        if let None = self.head {
            return false;
        }
        current = self.head.as_ref().unwrap().clone();

        loop {
            if current.clone().borrow().val == target {
                return true;
            } else if current.clone().borrow().next.is_none() ||
                current.clone().borrow().next.as_ref().unwrap().borrow().val > target {
                if current.borrow().down.is_none() {
                    return false;
                } else {
                    current = current.clone().borrow().down.as_ref().unwrap().clone();
                }
            } else {
                current = current.clone().borrow().next.as_ref().unwrap().clone();
            }
        }
    }

    pub fn insert(&mut self, val: T) {
        if self.head.is_none() {
            self.head = Some(Rc::new(RefCell::new(SkipListNode::new(val))));
            return;
        }

        let mut insert_level = 1;
        while insert_level <= Self::MAX_LEVEL {
            let rd = self.rng.gen_range(0..100);
            if rd >= Self::PROMOTION_PROBABILITY {
                insert_level += 1;
            } else {
                break;
            }
        }

        if insert_level > self.level {
            for _ in 0..insert_level - self.level {
                let head = self.head.as_ref().unwrap().clone();
                self.head = Some(Rc::new(RefCell::new(SkipListNode {
                    val,
                    next: None,
                    down: Some(head),
                })));
            }
        }
    }

    fn insert_after(node: &SkipListNode<T>, val: T) {

    }
}

pub fn skiplist_test() {
    let mut s = SkipList::<i32>::new();
    s.head = Some(Rc::new(RefCell::new(SkipListNode {
        val: 1,
        next: Some(Rc::new(RefCell::new(SkipListNode {
            val: 2,
            next: None,
            down: None,
        }))),
        down: None,
    })));
    println!("{}", s.contain(3));
    let mut t;
    t = rand::thread_rng();
}