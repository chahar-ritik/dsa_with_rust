use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        fn is_valid(
            root: &Option<Rc<RefCell<TreeNode>>>,
            prev: &mut i64
        ) -> bool {

            if let Some(node) = root {
                let node = node.borrow();

                if !is_valid(&node.left, prev) {
                    return false;
                }

                if node.val as i64 <= *prev {
                    return false;
                }

                *prev = node.val as i64;

                is_valid(&node.right, prev)
            } else {
                true
            }
        }

        let mut prev = i64::MIN;
        is_valid(&root, &mut prev)
    }
}