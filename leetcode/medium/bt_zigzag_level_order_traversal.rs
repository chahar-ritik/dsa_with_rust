use std::rc::Rc;
use std::cell::RefCell;

use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
     let mut result = Vec::new();
     if root.is_none(){
           return result;
        }
     let mut level_n = VecDeque::new();
     let mut rev = false;
     let mut queue = VecDeque::new();
      
      queue.push_back(root);
      queue.push_back(None);
      while let Some(rnode) = queue.pop_front(){
        if let Some(node) = rnode{
            let node = node.borrow();
            

            if !rev{
               level_n.push_back(node.val);
            
            }else{
                level_n.push_front(node.val);
           
            }
            if node.left.is_some(){
            queue.push_back(node.left.clone());
            }
            if node.right.is_some(){
            queue.push_back(node.right.clone());
             
            }
        }else{
            result.push(level_n.iter().cloned().collect());
            level_n.clear();
            rev = !rev;
           if !queue.is_empty(){
            queue.push_back(None);
           }
        }
      }
      result
    }
}