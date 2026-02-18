use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Option<Rc<RefCell<Node>>>;
#[derive(Debug)]
struct Node {
    val: i32,
    prev: Option<Weak<RefCell<Node>>>,
    next: Link,
}
#[derive(Debug)]
struct DoublylinkedList {
    head: Link,
    tail: Link,
}

impl DoublylinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, val: i32) {
        let newnode = Rc::new(RefCell::new(Node {
            val,
            prev: None,
            next: self.head.clone(),
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&newnode));
                self.head = Some(newnode);
            }
            None => {
                self.tail = Some(newnode.clone());
                self.head = Some(newnode);
            }
        }
    }

    fn push_back(&mut self, val: i32) {
        let newnode = Rc::new(RefCell::new(Node {
            val,
            prev: None,
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                newnode.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(Rc::clone(&newnode));
                self.tail = Some(newnode)
            }
            None => {
                self.head = Some(newnode.clone());
                self.tail = Some(newnode);
            }
        }
    }
    fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            let mut old_head_ref = old_head.borrow_mut();

            if let Some(next) = old_head_ref.next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail = None;
            }
            old_head_ref.val
        })
    }

    fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|old_tail| {
            let mut old_tail_ref = old_tail.borrow_mut();

            if let Some(prev_weak) = old_tail_ref.prev.take() {
                if let Some(prev) = prev_weak.upgrade() {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                }
            } else {
                self.head = None;
            }

            old_tail_ref.val
        })
    }

    fn print_front(&self) {
        let mut curr = self.head.clone();

        while let Some(node) = curr {
            print!("{} ", node.borrow().val);

            curr = node.borrow().next.clone();
        }

        println!("")
    }

    fn print_back(&self) {
        let mut curr = self.tail.clone();

        while let Some(node) = curr {
            print!("{} ", node.borrow().val);

            curr = node.borrow().prev.as_ref().and_then(|weak| weak.upgrade())
        }
    }
}

fn main() {
    let mut dll = DoublylinkedList::new();

    dll.push_front(1);
    dll.push_front(2);
    dll.push_back(2);
    dll.push_back(3);
    dll.push_front(1);
    dll.push_front(2);
    dll.push_back(2);
    dll.push_back(3);

    println!("Pop front: {:?}", dll.pop_back());

    print!("Print from front: ");
    dll.print_front();

    println!("Pop back: {:?}", dll.pop_front());

    print!("Print from front: ");
    dll.print_back();
}
