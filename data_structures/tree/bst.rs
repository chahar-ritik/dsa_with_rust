use std::cell::RefCell;
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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
    let arr = vec![3, 2, 1, 5, 6, 4];

    let mut root = None;

    for v in arr {
        root = build_bst(root, v);
    }

    inorder_traverse(&root);
}

fn build_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        let new_node = Rc::new(RefCell::new(TreeNode::new(val)));

        return Some(new_node);
    }

    if let Some(node) = root.as_ref() {
        let mut node = node.borrow_mut();

        if node.val > val {
            node.left = build_bst(node.left.clone(), val);
        } else {
            node.right = build_bst(node.right.clone(), val);
        }
    }

    root
}

fn inorder_traverse(root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder_traverse(&node.left);
        println!("{}", node.val);
        inorder_traverse(&node.right);
    }
}
