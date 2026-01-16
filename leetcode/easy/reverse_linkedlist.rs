// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(
        mut head: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {

        let mut prev: Option<Box<ListNode>> = None;

        while let Some(mut node) = head {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            head = next;
        }

        prev
    }
}
