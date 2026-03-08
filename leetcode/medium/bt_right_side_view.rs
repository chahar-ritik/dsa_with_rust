/*use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque; */
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        if root.is_none(){
           return result;
        }
          queue.push_back(root.clone());
          queue.push_back(None);

          while let Some(rnode) = queue.pop_front(){
             if let Some(node) = rnode{
                let node = node.borrow();
                if let Some(val) = queue.front(){
                   if val.is_none(){
                      result.push(node.val);
                   }
                  }

                if node.left.is_some(){
                queue.push_back(node.left.clone());
                }
                  if node.right.is_some(){
                queue.push_back(node.right.clone());
                  }

                  
             }else{
                  if !queue.is_empty(){
                    queue.push_back(None);
                  }
             }
          }
          result
    }