use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
         let mut result = Vec::new();
       
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>> , result : &mut Vec<i32> ){
            if let Some(node) = root{
                let node = node.borrow();
                dfs(&node.left,result);
                dfs(&node.right,result);
                result.push(node.val);

            }
        }
         dfs(&root , &mut result);
         result
    }
}