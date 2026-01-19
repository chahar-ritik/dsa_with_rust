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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }

       let mut anslist = Box::new(ListNode::new(0));
       anslist.next = head; 
       let mut prev = &mut anslist;
        
        //skip untill we have position less then left
       for _ in 1..left{
        prev = prev.next.as_mut().unwrap()
       }

       let mut curr = prev.next.take();
       let mut rev : Option<Box<ListNode>> = None ;

        //reverse from left to right
       for _ in left..=right {
        if let Some(mut node) = curr{
            curr = node.next.take();
            node.next = rev;
            rev = Some(node);
        }
       }

     // attach to previous or node brfore left
      prev.next = rev;
    
    // add nodes after right
    let mut p = prev;
    while p.next.is_some(){
      p =  p.next.as_mut().unwrap();
    }


     p.next = curr;
 
     anslist.next

    }
}