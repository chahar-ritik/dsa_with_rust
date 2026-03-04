impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0_i64; n + 1];
        let mut stack = Vec::new();
        let mut maxsub: i64 = 0;
        let modulo: i64 = 1000000007;

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
        }

        for i in 0..=n {
            let currval = if i == n { 0 } else { nums[i] };
            while let Some(&top) = stack.last() {
                if currval <= nums[top] {
                    stack.pop();
                    let minele = nums[top] as i64;
                    if let Some(&prev) = stack.last() {
                        let subsum = prefix_sum[i] - prefix_sum[prev + 1];

                        maxsub = maxsub.max(subsum * minele);
                    } else {
                        let subsum = prefix_sum[i];
                        maxsub = maxsub.max(subsum * minele);
                    }
                } else {
                    break;
                }
            }

            if i < n {
                stack.push(i);
            }
        }

        (maxsub % modulo) as i32
    }
}
