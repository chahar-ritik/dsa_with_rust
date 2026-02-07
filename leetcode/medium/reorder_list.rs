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
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return;
        }
        
        
        let mut fast = head.as_ref();
        let mut count = 0;
      
        
        
        while fast.is_some() && fast.unwrap().next.is_some() {
           count +=1;
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        
       let mut mid = head.as_mut();

       for i in 0..count{
        mid = mid.unwrap().next.as_mut();
       }
         let mut second_half = mid.as_mut().unwrap().next.take();
        let mut prev = None;
        let mut curr = second_half;
        
      
        while let Some(mut node) = curr {
            let next_temp = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next_temp;
        }
        let mut second = prev;
        
       
        let mut first = head;
        
        while second.is_some() {
           
            let mut first_node = first.as_mut().unwrap();
            let mut second_node = second.as_mut().unwrap();
            
          
            let temp1 = first_node.next.take();
            let temp2 = second_node.next.take();
            
           
            first_node.next = second.take();
            first_node.next.as_mut().unwrap().next = temp1;
            
           
            first = &mut first_node.next.as_mut().unwrap().next;
            second = temp2;
        }
    }
}
