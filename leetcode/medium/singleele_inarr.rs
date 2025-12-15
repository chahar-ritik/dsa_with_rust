impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
         let mut st = 0;
        let mut end = nums.len() - 1;

        while st < end {
            let mut mid = st + (end - st) / 2;

            // make mid even
            if mid % 2 == 1 {
                mid -= 1;
            }

            if nums[mid] == nums[mid + 1] {
                
                st = mid + 2;
            } else {
                
                end = mid;
            }
        }

        nums[st]
    }
}