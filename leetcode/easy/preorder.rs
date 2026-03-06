//leetcode problem 
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(rootnode) = root{
             let rootnode = rootnode.borrow();
             let left = Self::preorder_traversal(rootnode.left.clone());
             let right = Self::preorder_traversal(rootnode.right.clone());

             [vec![rootnode.val],left,right].concat()
        }else{
            Vec::new()
        }
    }
}


/*  [vec![rootnode.val],left,right].concat() 
let us assume we have root node 1 and its left childe node is 2 so we will get something left that itself a Vec<i32> as that have some value else we get empty Vec we have [vec![1] , vec![2] , Vec::new() , Vec::new() , Vec::new() ] and .concat() give us [1,2] Vec<Vec<i32>> -> Vec<i32>*/