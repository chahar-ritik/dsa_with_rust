use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>> ;
struct TreeNode{
    val: i32,
    left: Node,
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
    preorder_rc(&node);
    
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
   } // you can drop manually if don't want block as it is doing same drop(node_borrow);
   Some(node)
}

fn preorder_rc(node: &Node) {
    if let Some(rnode) = node {
        print!("{} ", rnode.borrow().val);
        preorder_rc(&rnode.borrow().left);
        preorder_rc(&rnode.borrow().right);
    }
}