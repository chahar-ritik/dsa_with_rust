//LRU stand for least recently used 
//It is not about a dsa problem it is about design of working cache memory

use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::collections::HashMap;

type Link = Option<Rc<RefCell<Node>>>;

struct Node{
    key:i32,
    val:i32,
    prev:Option<Weak<RefCell<Node>>>,
    next:Link,

}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32,Rc<RefCell<Node>>>,
    head: Link,
    tail: Link,

}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Node {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(Node::new(-1,-1)));
        let tail = Rc::new(RefCell::new(Node::new(-1,-1)));
       
       head.borrow_mut().next = Some(tail.clone());
       tail.borrow_mut().prev = Some(Rc::downgrade(&head));

       Self{
        capacity: capacity as usize,
        map: HashMap::new(),
        head: Some(head),
        tail: Some(tail),
       }
      
        
    }
    
    fn get(&mut self, key: i32) -> i32 {
          if let Some(node) = self.map.get(&key).cloned(){           
            self.delete(node.clone());
            self.add_to_front(node.clone());
            self.map.insert(key,node.clone());
        return node.borrow().val
          }else{
            -1
          }

        
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned(){
        
                self.delete(node.clone());
                 node.borrow_mut().val = value;
                

                 self.add_to_front(node.clone());

                 self.map.insert(key,node);
            
        }
        else {
            if self.capacity <= self.map.len(){
           let last_node = self.tail.as_ref()
               .unwrap()
               .borrow()     
                .prev         
               .as_ref()      
                .unwrap()
                .upgrade()     
                .unwrap();    

                self.delete(last_node);

            }
              let new_node = Rc::new(RefCell::new(Node::new(key,value)));

                 self.add_to_front(new_node.clone());

                 self.map.insert(key,new_node);
                  }
    }

    fn delete(&mut self,node: Rc<RefCell<Node>>){
        let prev = node.borrow().prev.clone().and_then(|w| w.upgrade());
        let next = node.borrow().next.clone();
        
        if let (Some(prev_node),Some(next_node)) = (prev,next){
            prev_node.borrow_mut().next = Some(next_node.clone());
            next_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));

        }

        {
            let mut node_ref = node.borrow_mut();
             node_ref.next = None;
             node_ref.prev = None;
        }

        self.map.remove(&node.borrow().key);
    }

    fn add_to_front(&self,node: Rc<RefCell<Node>>) {

       let prev = self.head.clone();
       let next = self.head.as_ref().unwrap().borrow().next.clone();
          
        if let (Some(prev_node),Some(next_node)) = (prev,next){

            {
            let mut node_ref = node.borrow_mut();
            node_ref.prev = Some(Rc::downgrade(&prev_node));
            node_ref.next = Some(next_node.clone());
            }
            prev_node.borrow_mut().next = Some(node.clone());
            next_node.borrow_mut().prev = Some(Rc::downgrade(&node));

        }

    }
}