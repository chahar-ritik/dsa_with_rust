fn move_to_front(
    head: &mut Option<Rc<RefCell<Node>>>,
    tail: &mut Option<Rc<RefCell<Node>>>,
    node: Rc<RefCell<Node>>,
) {
    let prev = node.borrow().prev.clone().and_then(|w| w.upgrade());
    let next = node.borrow().next.clone();


    // No need to traverse with current as the node is part of linked list and we have node pointer

    match (prev, next) {
        (None, None) => {
            return;
        }

        (None, Some(next_node)) => {
            return;
        }
        (Some(prev_node), None) => {
            prev_node.borrow_mut().next = None;
            *tail = Some(prev_node.clone());
        }

        (Some(prev_node), Some(next_node)) => {
            prev_node.borrow_mut().next = Some(next_node.clone());
            next_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
        }
    }
    if let Some(old_head) = head.clone() {
        {
            let mut node_ref = node.borrow_mut();
            node_ref.prev = None;
            node_ref.next = Some(old_head.clone());
        }

        old_head.borrow_mut().prev = Some(Rc::downgrade(&node));
    }

    *head = Some(node);
}
