use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut prefix = vec![0; nums.len()];
        prefix[0] = nums[0];

        for i in 1..nums.len() {
            prefix[i] = prefix[i - 1] + nums[i];
        }

        let mut map = HashMap::new();
        let mut count = 0;

        for sum in prefix {
           
            if sum == k {
                count += 1;
            }

            let need = sum - k;

           
            if let Some(v) = map.get(&need) {
                count += v;
            }

         
            *map.entry(sum).or_insert(0) += 1;
        }

        count
    }
}

//optimal and more same
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);

        let mut prefix = 0;
        let mut count = 0;

        for n in nums {
            prefix += n;

            if let Some(v) = map.get(&(prefix - k)) {
                count += v;
            }

            *map.entry(prefix).or_insert(0) += 1;
        }

        count
    }
}

