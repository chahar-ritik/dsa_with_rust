impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let modulo: i64 = 1_000_000_007;

        // Precompute nCr using Pascal Triangle
        let mut comb = vec![vec![0i64; n + 1]; n + 1];
        for i in 0..=n {
            comb[i][0] = 1;
            comb[i][i] = 1;
            for j in 1..i {
                comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % modulo;
            }
        }

        fn dfs(nums: &Vec<i32>, comb: &Vec<Vec<i64>>, modulo: i64) -> i64 {
            if nums.len() <= 2 {
                return 1;
            }

            let root = nums[0];

            let mut left = Vec::new();
            let mut right = Vec::new();

            for &num in nums.iter().skip(1) {
                if num < root {
                    left.push(num);
                } else {
                    right.push(num);
                }
            }

            let left_ways = dfs(&left, comb, modulo);
            let right_ways = dfs(&right, comb, modulo);

            let l = left.len();
            let r = right.len();

            let ways = comb[l + r][l]; 

            (((ways * left_ways) % modulo) * right_ways) % modulo
        }

        let result = dfs(&nums, &comb, modulo);

        ((result - 1 ) % modulo) as i32
    }
}