use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub prev: Option<Weak<RefCell<Node>>>,
}

pub fn insert_at(
    mut head: Option<Rc<RefCell<Node>>>,
    index: usize,
    value: i32,
) -> Option<Rc<RefCell<Node>>> {
    let new_node = Rc::new(RefCell::new(Node {
        val: value,
        prev: None,
        next: None,
    }));

    if head.is_none() {
        return Some(new_node);
    }

    if index == 0 {
        if let Some(old_head) = head.clone() {
            new_node.borrow_mut().next = Some(old_head.clone());
            old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        }
        return Some(new_node);
    }
    let mut curr = head.clone();
    let mut count = 0;

    while let Some(node) = curr.clone() {
        if index == count {
            // prev node
            let prev = node.borrow().prev.clone().and_then(|w| w.upgrade());

            if let Some(prev_node) = prev {
                prev_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
            }

            //next node
            node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            new_node.borrow_mut().next = Some(node.clone());

            return head;
        }

        count += 1;
        curr = node.borrow().next.clone();
    }

    if let Some(mut tail) = head.clone() {
        while let Some(next) = tail.borrow().next.clone() {
            tail = next;
        }

        tail.borrow_mut().next = Some(new_node.clone());
        new_node.borrow_mut().prev = Some(Rc::downgrade(&tail));
    }
    return head;
}
