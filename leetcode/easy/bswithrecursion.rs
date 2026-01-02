
fn binsearch(nums: &[i32], target: i32, st: usize, end: usize) -> i32 {
    if st >= end {
        return -1;
    }

    let mid = st + (end - st) / 2;

    if nums[mid] == target {
        return mid as i32;
    } else if nums[mid] < target {
        binsearch(nums, target, mid + 1, end)
    } else {
        binsearch(nums, target, st, mid)
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        binsearch(&nums, target, 0, nums.len())
    }
}
