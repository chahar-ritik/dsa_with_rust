use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut heap = BinaryHeap::new();
        let mut curr_max = i32::MIN;

       
        for r in 0..n {
            heap.push(Reverse((nums[r][0], r, 0)));
            curr_max = curr_max.max(nums[r][0]);
        }

        let mut res = vec![0, i32::MAX];

        while let Some(Reverse((val, r, c))) = heap.pop() {

           
            if curr_max - val < res[1] - res[0] {
                res[0] = val;
                res[1] = curr_max;
            }

           // break when a list from k end
            if c + 1 >= nums[r].len() {
                break;
            }

           
            let next_val = nums[r][c + 1];
            heap.push(Reverse((next_val, r, c + 1)));

           
            curr_max = curr_max.max(next_val);
        }

        res
    }
}