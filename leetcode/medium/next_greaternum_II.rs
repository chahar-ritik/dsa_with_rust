impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..(2 * n) {
            let idx = i % n;

            while let Some(&top) = stack.last() {
                if nums[idx] > nums[top] {
                    stack.pop();
                    ans[top] = nums[idx];
                } else {
                    break;
                }
            }

            if i < n {
                stack.push(idx);
            }
        }

        ans
    }
}
