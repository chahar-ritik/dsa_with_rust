use std::rc::Rc;
use std::cmp::max;
use std::cell::RefCell;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        helper(&root , &mut max_sum);
        fn helper(root : &Option<Rc<RefCell<TreeNode>>> , max_sum: &mut i32  ) -> i32{
            if let Some(rnode) = root{
                let rnode = rnode.borrow();
                let left_max = helper(&rnode.left , max_sum);
                let right_max = helper(&rnode.right , max_sum);
                let curr_val = rnode.val;
                *max_sum = (*max_sum).max(curr_val + max(0,left_max) + max(0,right_max));
               curr_val + max(max(left_max,right_max),0)
            }else{
                0
            }
        }
        max_sum
    }
}