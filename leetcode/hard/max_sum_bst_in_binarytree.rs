use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        fn postorder(
            node: &Option<Rc<RefCell<TreeNode>>>,
            max_sum: &mut i32
        ) -> (bool, i32, i32, i32) {

          
            if node.is_none() {
                return (true, i32::MAX, i32::MIN, 0);
            }

            let n = node.as_ref().unwrap().borrow();

            let (l_bst, l_min, l_max, l_sum) = postorder(&n.left, max_sum);
            let (r_bst, r_min, r_max, r_sum) = postorder(&n.right, max_sum);

          
            if l_bst && r_bst && l_max < n.val && n.val < r_min {
                let sum = l_sum + r_sum + n.val;

               
                *max_sum = (*max_sum).max(sum);

                let min = l_min.min(n.val);
                let max = r_max.max(n.val);

                return (true, min, max, sum);
            }

           
            (false, 0, 0, 0)
        }

        let mut max_sum = 0;
        postorder(&root, &mut max_sum);

        max_sum
    }
}