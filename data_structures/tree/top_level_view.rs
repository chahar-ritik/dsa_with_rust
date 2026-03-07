use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;
struct TreeNode {
    val: i32,
    left: Node,
    right: Node,
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
    let preorder_seq = vec![
        1, 5, -1, -1, 7, 4, -1, -1, 2, 12, -1, 13, -1, 14, -1, 18, -1, -1, 11, -1, -1,
    ];
    let mut index = 0;
    let node = build_tree(&preorder_seq, &mut index);
    let top_view = top_level_view(&node);
    println!("top view");
    for i in top_view {
        print!("{} ", i);
    }
}

fn build_tree(preorder_seq: &Vec<i32>, index: &mut usize) -> Node {
    if preorder_seq[*index] == -1 {
        *index += 1;
        return None;
    }
    let node = Rc::new(RefCell::new(TreeNode::new(preorder_seq[*index])));
    *index += 1;
    {
        let mut node_borrow = node.borrow_mut();
        node_borrow.left = build_tree(preorder_seq, index);
        node_borrow.right = build_tree(preorder_seq, index);
    } // you can drop manually if don't want block as it is doing same drop(node_borrow);
    Some(node)
}

fn top_level_view(root: &Node) -> Vec<i32> {
    let mut queue = VecDeque::new();
    let mut bmap = BTreeMap::new();
    queue.push_back((root.clone(), 0));
    while let Some((rnode, vertical_line)) = queue.pop_front() {
        if let Some(node) = rnode {
            let node = node.borrow();
            bmap.entry(vertical_line).or_insert(node.val);

            queue.push_back((node.left.clone(), vertical_line - 1));
            queue.push_back((node.right.clone(), vertical_line + 1));
        }
    }

    bmap.values().cloned().collect()
}
