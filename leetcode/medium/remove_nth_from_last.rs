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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut fast = dummy.clone();
    let mut slow = &mut dummy;// this will return &mut Box<ListNode> and box apply deref trait so that makes it &mut ListNode by auto deref
   // we can also use dummy.as_mut() this is same


    for _ in 0..n {
        fast = fast.next.unwrap();
    }

    
    while let Some(next_node) = fast.next {
        fast = next_node;
        slow = slow.next.as_mut().unwrap(); // but here we can use &mut slow.next.unwrap() that gives us &mut Option<Box<<>> and unwrap can't be used with &mut Option<>
    }

    
    let to_be_deleted = slow.next.take();
    slow.next = to_be_deleted.and_then(|node| node.next);

    dummy.next
}
}