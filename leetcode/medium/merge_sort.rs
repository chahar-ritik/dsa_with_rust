impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }

        let end = nums.len() - 1;
        divide_arr(&mut nums, 0, end);
        nums
    }
}

fn divide_arr(nums: &mut Vec<i32>, st: usize, end: usize) {
    if st < end {
        let mid = st + (end - st) / 2;
        divide_arr(nums, st, mid);
        divide_arr(nums, mid + 1, end);

        merge(nums, st, mid, end);
    }
}

fn merge(nums: &mut Vec<i32>, st: usize, mid: usize, end: usize) {
    let mut i = st;
    let mut j = mid + 1;
    let mut temp = Vec::new();
    while i <= mid && j <= end {
        if nums[i] < nums[j] {
            temp.push(nums[i]);
            i += 1;
        } else {
            temp.push(nums[j]);
            j += 1;
        }
    }
    while i <= mid {
        temp.push(nums[i]);
        i += 1;
    }
    while j <= end {
        temp.push(nums[j]);
        j += 1;
    }

    for idx in 0..temp.len() {
        nums[idx + st] = temp[idx];
    }
}
