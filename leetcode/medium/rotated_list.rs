pub fn rotate_right(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Option<Box<ListNode>> {

        if head.is_none() || k == 0 {
            return head;
        }

     
        let mut len = 0;
        {
            let mut curr = head.as_ref();
            while let Some(node) = curr {
                len += 1;
                curr = node.next.as_ref();
            }
        }    

        let k = (k as usize) % len;
        if k == 0 {
            return head;
        }
         let mut rotatedlist = Box::new(ListNode::new(0));
        let mut steps = len - k ;


      
        let mut tail_ref =  head.as_mut();
        
         while let Some(mut tail) = tail_ref{
            steps -=1;
            if steps <= 0 {
                rotatedlist.next = tail.next.take();
            }
            tail_ref = tail.next.as_mut();
         }
        

        let mut curr_r = &mut rotatedlist;

        while curr_r.next.is_some(){
            curr_r = curr_r.next.as_mut().unwrap();
        }
        curr_r.next = head;
        rotatedlist.next
    }