    pub fn get_intersection_node(head_a: Link, head_b: Link) -> Link {
        let mut p1 = head_a.clone();
        let mut p2 = head_b.clone();

        while !Self::same_node(&p1, &p2) {
            p1 = match p1 {
                Some(ref node) => node.borrow().next.clone(),
                None => head_b.clone(),
            };

            p2 = match p2 {
                Some(ref node) => node.borrow().next.clone(),
                None => head_a.clone(),
            };
        }

        p1
    }

    fn same_node(a: &Link, b: &Link) -> bool {
        match (a, b) {
            (Some(x), Some(y)) => Rc::ptr_eq(x, y),
            (None, None) => true,
            _ => false,
        }
    }