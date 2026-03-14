use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> Option<i32> {
            if let Some(node) = root {
                let node = node.borrow();

                if let Some(val) = helper(&node.left, k) {
                    return Some(val);
                }

                *k -= 1;
                if *k == 0 {
                    return Some(node.val);
                }

                return helper(&node.right, k);
            }

            None
        }

        helper(&root, &mut k).unwrap()
    }
}