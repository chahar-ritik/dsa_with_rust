use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
     let mut map = HashMap::new();
    
    for (idx , &val) in inorder.iter().enumerate(){
        map.insert(val , idx);
    }
      
      let mut pre_index:usize  = 0;

      fn helper(preorder: &Vec<i32> , pre_index: &mut usize,left_b : usize ,right_b : usize , map: &HashMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>>{
        if left_b > right_b{
            return None;
        } 
        let idx_inor = *map.get(&preorder[*pre_index]).unwrap();
        let new_node = Rc::new(RefCell::new(TreeNode::new(preorder[*pre_index])));

        *pre_index+=1;
        {
            let mut new_node = new_node.borrow_mut();
            if idx_inor > 0{
            new_node.left = helper(preorder ,  pre_index , left_b , idx_inor-1, map);
            }
             new_node.right = helper(preorder ,  pre_index , idx_inor+1 , right_b ,map);
        }

        Some(new_node)
      }

       helper(&preorder , &mut pre_index ,0 , inorder.len()-1 , &mut map)   
    }
}