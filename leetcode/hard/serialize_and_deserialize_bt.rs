use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    // SERIALIZE
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = Vec::new();
        Self::dfs(root, &mut result);
        result.join(",")
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<String>) {
        match node {
            None => res.push("null".to_string()),
            Some(n) => {
                let node_ref = n.borrow();

                res.push(node_ref.val.to_string());
                Self::dfs(node_ref.left.clone(), res);
                Self::dfs(node_ref.right.clone(), res);
            }
        }
    }

    // DESERIALIZE
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue: VecDeque<&str> = data.split(',').collect();
        Self::build(&mut queue)
    }

    fn build(queue: &mut VecDeque<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        let val = queue.pop_front()?;

        if val == "null" {
            return None;
        }

        let node = Rc::new(RefCell::new(TreeNode {
            val: val.parse().unwrap(),
            left: None,
            right: None,
        }));

        node.borrow_mut().left = Self::build(queue);
        node.borrow_mut().right = Self::build(queue);

        Some(node)
    }
}