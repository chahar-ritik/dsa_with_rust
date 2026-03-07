use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>> ;
struct TreeNode{
    val: i32,
    left: Node ,
    right: Node,

}

impl TreeNode{
    fn new(val: i32) -> Self{
        Self{
            val,
            left: None,
            right: None,
        }
    }
}

fn main(){
    let preorder_seq = vec![1,2,-1,-1,3,4,-1,-1,5,-1,-1];
    let mut index = 0;
    let node = build_tree(&preorder_seq , &mut index);
    let height = height_of_bt(&node);

    println!("Height of binary tree: {}", height);
}

fn build_tree(preorder_seq:&Vec<i32> , index: &mut usize) -> Node{
    if preorder_seq[*index] == -1 {
          *index += 1;
        return None;
    }
   let node = Rc::new(RefCell::new(TreeNode::new(preorder_seq[*index])));
   *index+=1;
   {
   let mut node_borrow = node.borrow_mut();
   node_borrow.left = build_tree(preorder_seq,index);
   node_borrow.right = build_tree(preorder_seq,index);
   } 
   Some(node)
}


fn height_of_bt(node: &Node) -> i32{
    if node.is_none(){
        return 0;
    }
    let node_borrow = node.as_ref().unwrap().borrow();
    let left_tree = height_of_bt(&node_borrow.left);
    let right_tree = height_of_bt(&node_borrow.right);
     
     let height = left_tree.max(right_tree)+1;
     height
}