use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        fn helper(
            node: Option<Rc<RefCell<TreeNode>>>,
            prev: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(node_rc) = node {

                let (left, right) = {
                    let node = node_rc.borrow();
                    (node.left.clone(), node.right.clone())
                };

                if let Some(p) = prev.as_ref() {
                    let mut p = p.borrow_mut();
                    p.left = None;
                    p.right = Some(node_rc.clone());
                }

                *prev = Some(node_rc.clone());

                helper(left, prev);
                helper(right, prev);
            }
        }

        helper(root.clone(), &mut prev);
    }
}