use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue : VecDeque<(Option<Rc<RefCell<TreeNode>>>,i32)> = VecDeque::new();
        let mut max = 0;
        queue.push_back((root.clone(),0));
           while !queue.is_empty(){
            let st_idx = queue.front().unwrap().1;
            let end_idx = queue.back().unwrap().1;
            max = max.max(end_idx-st_idx+1);
            let n = queue.len();
             for i in 0..n{
                if let Some((node_rc , idx)) = queue.pop_front(){
                    if let Some(node) = node_rc{
                        let node = node.borrow();
                        if node.left.is_some(){
                        queue.push_back((node.left.clone(),(2*idx as i32+1)));
                        //as first idx start from 0 for root and then like are level wise left of root 2*0+1 =1 
                        }
                        if node.right.is_some(){
                        queue.push_back((node.right.clone(),(2*idx as i32+2)));
                        //root right node idx 2*0+2  
                        }
                    }
                }
             }
           }
           max
    }
}