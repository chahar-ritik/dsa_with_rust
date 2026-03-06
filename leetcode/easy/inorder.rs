use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
       
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>> , result : &mut Vec<i32> ){
            if let Some(node) = root{
                dfs(&node.borrow().left,result);
                result.push(node.borrow().val);
                dfs(&node.borrow().right,result);

            }
        }
         dfs(&root , &mut result);
         result
    }
}