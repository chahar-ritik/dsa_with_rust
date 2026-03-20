use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_merge(
        trees: Vec<Option<Rc<RefCell<TreeNode>>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {

        let mut root_map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut leaf_set: HashSet<i32> = HashSet::new();

      
        for tree in &trees {
            if let Some(node) = tree {
                let node_ref = node.borrow();
                let val = node_ref.val;

                root_map.insert(val, node.clone());

                if let Some(left) = node_ref.left.clone() {
                    leaf_set.insert(left.borrow().val);
                }
                if let Some(right) = node_ref.right.clone() {
                    leaf_set.insert(right.borrow().val);
                }
            }
        }

        
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;

        for (val, node) in &root_map {
            if !leaf_set.contains(val) {
                root = Some(node.clone());
                break;
            }
        }

        let root = root?;

       
        fn dfs(
            node: Rc<RefCell<TreeNode>>,
            min: i64,
            max: i64,
            root_map: &mut HashMap<i32, Rc<RefCell<TreeNode>>>
        ) -> bool {
            let val = node.borrow().val as i64;

           
            if val <= min || val >= max {
                return false;
            }

          
            {
                let mut node_mut = node.borrow_mut();

                if node_mut.left.is_none() && node_mut.right.is_none() {
                    if let Some(subtree) = root_map.remove(&node_mut.val) {
                      
                        if !Rc::ptr_eq(&node, &subtree) {
                            let sub_ref = subtree.borrow();
                            node_mut.left = sub_ref.left.clone();
                            node_mut.right = sub_ref.right.clone();
                        }
                    }
                }
            }

         
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            if let Some(l) = left {
                if !dfs(l, min, val, root_map) {
                    return false;
                }
            }

            if let Some(r) = right {
                if !dfs(r, val, max, root_map) {
                    return false;
                }
            }

            true
        }

      
        root_map.remove(&root.borrow().val);

     
        if !dfs(root.clone(), i64::MIN, i64::MAX, &mut root_map) {
            return None;
        }

       
        if !root_map.is_empty() {
            return None;
        }

        Some(root)
    }
}