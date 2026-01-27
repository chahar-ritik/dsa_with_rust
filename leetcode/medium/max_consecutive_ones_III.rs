impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut zeros = 0;
        let mut ans = 0;
        let k = k as usize;

        for r in 0..nums.len() {
            if nums[r] == 0 {
                zeros += 1;
            }

            while zeros > k {
                if nums[l] == 0 {
                    zeros -= 1;
                }
                l += 1;
            }

            ans = ans.max(r - l + 1);
        }

        ans as i32
    }
}
