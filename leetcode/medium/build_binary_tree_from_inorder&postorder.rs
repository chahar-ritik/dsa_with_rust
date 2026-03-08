use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();

        for (i,&v) in inorder.iter().enumerate(){
            map.insert(v,i);
        } 

        let mut post_rev_index = postorder.len()-1;

        fn helper(postorder: &Vec<i32> , post_rev_index: &mut usize , in_left: usize , in_right: usize , map : &HashMap<i32,usize>) -> Option<Rc<RefCell<TreeNode>>>{
            if in_left > in_right  {
                return None;
            }
           let inorder_idx = *map.get(&postorder[*post_rev_index]).unwrap();
           let new_node = Rc::new(RefCell::new(TreeNode::new(postorder[*post_rev_index])));
          if *post_rev_index > 0{
            *post_rev_index-=1;
          }
          
          {
            let mut new_node = new_node.borrow_mut();
              new_node.right = helper(postorder , post_rev_index , inorder_idx+1 , in_right  , map);
          if inorder_idx > in_left {
           new_node.left = helper(postorder , post_rev_index , in_left , inorder_idx -1 ,map);
          }
          
        
          }
          Some(new_node)
        } 

        helper(&postorder , &mut post_rev_index , 0 , inorder.len() -1 ,&map)
    }
}