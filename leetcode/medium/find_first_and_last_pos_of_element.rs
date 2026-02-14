impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn find_left(nums: &Vec<i32>, target: i32) -> i32 {
            let (mut left, mut right) = (0, nums.len() as i32 - 1);
            let mut ans = -1;

            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[mid as usize] >= target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
                if nums[mid as usize] == target {
                    ans = mid;
                }
            }

            ans
        }

        fn find_right(nums: &Vec<i32>, target: i32) -> i32 {
            let (mut left, mut right) = (0, nums.len() as i32 - 1);
            let mut ans = -1;

            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[mid as usize] <= target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
                if nums[mid as usize] == target {
                    ans = mid;
                }
            }

            ans
        }

        vec![find_left(&nums, target), find_right(&nums, target)]
    }
}
