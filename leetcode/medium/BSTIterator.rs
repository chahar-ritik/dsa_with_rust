// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,

}

impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();
        let mut curr = root;

        while let Some(node) = curr {
            stack.push(node.clone());
            curr = node.borrow().left.clone();
        }

        Self { stack }
    }
    
    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let val = node.borrow().val;

        let mut curr = node.borrow().right.clone();

        while let Some(n) = curr {
            self.stack.push(n.clone());
            curr = n.borrow().left.clone();
        }

        val
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}
