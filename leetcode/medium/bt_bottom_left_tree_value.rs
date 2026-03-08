use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0 ;
        let mut queue = VecDeque::new();
         if let Some(node) = root.as_ref(){
            ans = node.borrow().val;
         }
        queue.push_back(root.clone());
        queue.push_back(None);

        while let Some(rnode) = queue.pop_front(){
            if let Some(node) = rnode{
                let node = node.borrow();
                if node.left.is_some() {
                   queue.push_back(node.left.clone());
                  }

                  if node.right.is_some() {
                   queue.push_back(node.right.clone());
                    }


            }else{
                if let Some(Some(node)) = queue.front() {
                        ans = node.borrow().val;
                      }
                      if !queue.is_empty(){
                         queue.push_back(None);
                      }
                }
            
        }
         ans   
    }
}