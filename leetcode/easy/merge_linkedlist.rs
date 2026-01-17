impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, l2) => l2,
            (l1, None) => l1,

            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Solution::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Solution::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
        }
    }
}
