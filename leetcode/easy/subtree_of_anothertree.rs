use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
     fn check_same_tree(root: &Option<Rc<RefCell<TreeNode>>> , sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool{ 
        
        match (root , sub_root) {
            (Some(r_node) , Some(s_node)) => {
                let r_node = r_node.borrow();
                let s_node = s_node.borrow();
           r_node.val == s_node.val &&
                check_same_tree(&r_node.left , &s_node.left) && check_same_tree(&r_node.right , &s_node.right)
            }
            (None,None) => true,
            _=> false,
        }
     }

     fn dfs(root: &Option<Rc<RefCell<TreeNode>>> , sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool{
        if root.is_none(){
            return false;
        }

        if check_same_tree(&root , &sub_root){
            return true;
        }

        let root = root.as_ref().unwrap().borrow();

         dfs(&root.left ,&sub_root) ||  dfs(&root.right ,&sub_root)

     }
     dfs(&root ,&sub_root)
    }
}

// the below code works because it is leetcode rust specific defination of node as a struct 
//that implements trait PartialEq and Eq that allow to directly compare the tree as we compare integers 


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
impl Solution {
    fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let b = root == sub_root;
    if let Some(node_rc) = root {
        let node = node_rc.borrow();
        let lb = Self::is_subtree(node.left.clone(), sub_root.clone());
        let rb = Self::is_subtree(node.right.clone(), sub_root.clone());

        !(!lb && !rb && !b) // By De Morgan's law this will be lb||rb||b
    } else {
        false
    }
}
}