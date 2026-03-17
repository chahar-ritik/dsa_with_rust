use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(
            preorder: &Vec<i32>,
            idx: &mut usize,
            min: i32,
            max: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if *idx >= preorder.len() {
                return None;
            }

            let val = preorder[*idx];

            if val <= min || val >= max {
                return None;
            }

            *idx += 1;

            let node = Rc::new(RefCell::new(TreeNode::new(val)));

            node.borrow_mut().left = build(preorder, idx, min, val);
            node.borrow_mut().right = build(preorder, idx, val, max);

            Some(node)
        }

        let mut idx = 0;
        build(&preorder, &mut idx, i32::MIN, i32::MAX)
    }
}