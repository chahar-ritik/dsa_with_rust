//if two element sum in bst equal to k

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        
        fn push_left(mut node: Option<Rc<RefCell<TreeNode>>>,stack : &mut Vec<Rc<RefCell<TreeNode>>>) {
            while let Some(n) = node {
                stack.push(n.clone());
                node = n.borrow().left.clone();
            }
        }
        fn push_right(mut node: Option<Rc<RefCell<TreeNode>>>,stack : &mut Vec<Rc<RefCell<TreeNode>>>) {
            while let Some(n) = node {
                stack.push(n.clone());
                node = n.borrow().right.clone();
            }
        }

        let mut left_stack = Vec::new();
        let mut right_stack = Vec::new();

         push_left(root.clone() , &mut left_stack);
         push_right(root.clone() , &mut right_stack);

         let mut left = left_stack.pop();
          let mut right = right_stack.pop();
         
         while let (Some(l), Some(r)) = (&left , &right) {
            if Rc::ptr_eq(l , r){
                break
            }
            let sum = l.borrow().val + r.borrow().val;

            if sum == k {
                return true; 
            } else if sum < k{
               push_left(l.borrow().right.clone(), &mut left_stack);
                left = left_stack.pop();
            }else {
               push_right(r.borrow().left.clone(), &mut right_stack);
                right = right_stack.pop();
            }
         }
        

        false
    }
}