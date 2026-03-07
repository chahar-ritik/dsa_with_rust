use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
// TreeNode that use Box and Node that use Rc and RefCell
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {
    //Using Box
    let mut root = Box::new(TreeNode::new(1));
    let child_a = Box::new(TreeNode::new(2));

    root.left = Some(child_a);
    root.right = Some(Box::new(TreeNode::new(3)));
    let root = Some(root);
    println!("Using Box\n Inorder traversal");
    inorder(&root);
    println!("\nPreorder traversal");
    preorder(&root);
    println!("\nPostorder traversal");
    postorder(&root);

    //Using Rc and RefCell
    /*let rootnode = Rc::new(RefCell::new(Node::new(4)));
    let child_left = Rc::new(RefCell::new(Node::new(5)));
    let child_right = Rc::new(RefCell::new(Node::new(5)));

    rootnode.borrow_mut().left = Some(Rc::clone(&child_left));
    rootnode.borrow_mut().right = Some(Rc::clone(&child_right));
    let rootnode = Some(rootnode);
*/
    let preorder_seq = vec![1,2,-1,-1,3,4,-1,-1,5,-1,-1];
    let mut index = 0;
    let rootnode = build_tree(&preorder_seq , &mut index);

    println!("\nUsing Rc and RefCell\n Inorder traversal");
    inorder_rc(&rootnode);
    println!("\nPreorder traversal");
    preorder_rc(&rootnode);
    println!("\nPostorder traversal");
    postorder_rc(&rootnode);
     println!("\nLevel order traversal");
     level_order_traversal(&rootnode);
}

fn build_tree(preorder_seq:&Vec<i32> , index: &mut usize) -> Option<Rc<RefCell<Node>>>{
    if preorder_seq[*index] == -1 {
          *index += 1;
        return None;
    }
   let node = Rc::new(RefCell::new(Node::new(preorder_seq[*index])));
   *index+=1;
   {
   let mut node_borrow = node.borrow_mut();
   node_borrow.left = build_tree(preorder_seq,index);
   node_borrow.right = build_tree(preorder_seq,index);
   } // you can drop manually if don't want block as it is doing same drop(node_borrow);
   Some(node)
}

fn inorder(node: &Option<Box<TreeNode>>) {
    if let Some(rnode) = node {
        inorder(&rnode.left);
        print!("{} ", rnode.val);
        inorder(&rnode.right);
    }
}

fn preorder(node: &Option<Box<TreeNode>>) {
    if let Some(rnode) = node {
        print!("{} ", rnode.val);
        preorder(&rnode.left);
        preorder(&rnode.right);
    }
}

fn postorder(node: &Option<Box<TreeNode>>) {
    if let Some(rnode) = node {
        postorder(&rnode.left);
        postorder(&rnode.right);
        print!("{} ", rnode.val);
    }
}

//for leetcode problems
//using Rc and RefCell

struct Node {
    val: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn inorder_rc(node: &Option<Rc<RefCell<Node>>>) {
    if let Some(rnode) = node {
        inorder_rc(&rnode.borrow().left);
        print!("{} ", rnode.borrow().val);
        inorder_rc(&rnode.borrow().right);
    }
}

fn preorder_rc(node: &Option<Rc<RefCell<Node>>>) {
    if let Some(rnode) = node {
        print!("{} ", rnode.borrow().val);
        preorder_rc(&rnode.borrow().left);
        preorder_rc(&rnode.borrow().right);
    }
}

fn postorder_rc(node: &Option<Rc<RefCell<Node>>>) {
    if let Some(rnode) = node {
        postorder_rc(&rnode.borrow().left);
        postorder_rc(&rnode.borrow().right);
        print!("{} ", rnode.borrow().val);
    }
}

fn level_order_traversal(node: &Option<Rc<RefCell<Node>>>){
    let mut queue = VecDeque::new();
   
   
    queue.push_back(node.clone());
   
    while let Some(root) = queue.pop_front(){{
       
       if let Some(node) = root{
               
               
               print!("{} ", node.borrow().val);
               queue.push_back(node.borrow().left.clone());
               queue.push_back(node.borrow().right.clone());
           }
        }
    }
}