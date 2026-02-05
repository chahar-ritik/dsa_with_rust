//SC O(1) or using constant space extra space
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow: usize = 0;
        let mut fast: usize = 0;
        let n = nums.len();

        loop{
     slow = nums[slow]as usize;
     fast = nums[nums[fast]as usize] as usize;

        if slow == fast {
          break
         }
    }
        let mut ptr1 : usize= 0;
        let mut ptr2 : usize = slow;
        while ptr1 != ptr2 {
            ptr1 = nums[ptr1] as usize ;
            ptr2 = nums[ptr2] as usize;
        }
        return ptr1 as i32;
      
    }
}