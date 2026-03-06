use std::cell::RefCell;
use std::rc::Rc;

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
    let rootnode = Rc::new(RefCell::new(Node::new(4)));
    let child_left = Rc::new(RefCell::new(Node::new(5)));
    let child_right = Rc::new(RefCell::new(Node::new(5)));

    rootnode.borrow_mut().left = Some(Rc::clone(&child_left));
    rootnode.borrow_mut().right = Some(Rc::clone(&child_right));
    let rootnode = Some(rootnode);

    println!("\nUsing Rc and RefCell\n Inorder traversal");
    inorder_rc(&rootnode);
    println!("\nPreorder traversal");
    preorder_rc(&rootnode);
    println!("\nPostorder traversal");
    postorder_rc(&rootnode);
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
