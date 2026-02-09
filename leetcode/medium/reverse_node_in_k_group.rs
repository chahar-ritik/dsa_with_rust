 pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k <= 1{
            return head;
        }

        let mut dummy  = Box::new(
            ListNode{
                val: 0,
                next: head,
            });

        let mut prev_group_end = &mut dummy;

        loop{
            let mut count = 0;
            let mut temp  = prev_group_end.next.as_ref();

            while let Some(node) = temp{
                count += 1;
                if count == k{
                    break;
                }
                temp = node.next.as_ref()
            }

            if count < k{
                break;
            }

            let mut group_start = prev_group_end.next.take();
            let mut next_group_start = group_start.as_ref();

            for _ in 0..k{
                next_group_start = next_group_start.unwrap().next.as_ref();
            }

            let mut prev = next_group_start.cloned();
            let mut curr = group_start;


          for _ in 0..k{
            if let Some(mut node) = curr{
                let next = node.next.take();
                node.next = prev;
                prev = Some(node);
                curr = next;
            }
          }

          prev_group_end.next = prev;

          for _ in 0..k{
            prev_group_end = prev_group_end.next.as_mut().unwrap();
          }

        }

        dummy.next
    }