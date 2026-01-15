impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some(){
                slow = &slow.as_ref().unwrap().next;
                fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
                }

                slow.clone()

        }
        
    }

    // slow and fast two pointers when fast will be at end slow will be at middle and we will return that