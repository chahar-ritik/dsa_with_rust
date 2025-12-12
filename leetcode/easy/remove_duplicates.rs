impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            let mut i = 1;
    while i < nums.len() {
        if nums[i] == nums[i - 1] {
            nums.remove(i);
        } else {
            i += 1;
        }
    }
    nums.len() as i32

    }
}


// Time complexity is O(n^2) because of remove() method
// two pointers approach can be used for more optimized solution