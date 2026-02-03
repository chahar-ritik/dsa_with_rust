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
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        if head.is_none() || head.as_ref().unwrap().next.is_none(){
            return head;
        }

        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.unwrap().next.is_some() && fast.unwrap().next.as_ref().unwrap().next.is_some(){

            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }

       
        let mut second = slow.unwrap().next.clone();
        let mut curr = &mut head;

        while curr.as_ref().unwrap().next.as_ref() != second.as_ref(){
            curr = &mut curr.as_mut().unwrap().next;
        }

        let second = curr.as_mut().unwrap().next.take();

        let left = Solution::sort_list(head);
        let right = Solution::sort_list(second);

        Solution::merge(left,right)
        
    }

    fn merge(mut l1: Option<Box<ListNode>> , mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while l1.is_some() && l2.is_some() {
            let take_l1 = l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val;

            let node = if take_l1{
                let mut n = l1.take().unwrap();
                l1 = n.next.take();
                n
            }else{
                let mut n = l2.take().unwrap();
                l2 = n.next.take();
                n
            };

            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if l1.is_some(){l1}else{l2};
        dummy.next
    }

}