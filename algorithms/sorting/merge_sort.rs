fn merge_sort(nums: &mut Vec<i32>, st: usize, end: usize) {
    if st < end {
        let mid = st + (end - st) / 2;
        let left = merge_sort(nums, st, mid);
        let right = merge_sort(nums, mid + 1, end);

        merge(nums, st, mid, end);
    }
}

fn merge(nums: &mut Vec<i32>, st: usize, mid: usize, end: usize) {
    let mut temp: Vec<i32> = Vec::new();

    let mut left = st;
    let mut right = mid + 1;

    while left <= mid && right as usize <= end {
        if nums[left as usize] <= nums[right as usize] {
            temp.push(nums[left]);
            left += 1;
        } else {
            temp.push(nums[right]);
            right += 1;
        }
    }

    while left <= mid {
        temp.push(nums[left]);
        left += 1;
    }

    while right <= end {
        temp.push(nums[right]);
        right += 1;
    }

    for idx in 0..temp.len() {
        nums[idx + st] = temp[idx];
    }
}

fn main() {
    let mut nums: Vec<i32> = vec![12, 31, 35, 8, 32, 17];
    if nums.len() > 1 {
        let n = nums.len() - 1;
        merge_sort(&mut nums, 0, n);
        println! {"{:?}",nums};
    } else {
        println! {"{:?}",nums};
    }
}
