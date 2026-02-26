
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
   let (a,b) = if nums1.len() <= nums2.len(){
      (nums1,nums2)
   }else{
      (nums2,nums1)
   }
   
   let (m,n) = (nums1.len(0),nums2.len());

   let mut left = 0;
   let mut right = m;

   while left <= right{
    let i = m/2;
    let j = m+n+1/2 - i;

    
    let max_left_a = if i == 0 {i32::MIN}else {a[i-1]};
    let min_right_a = if i == m {i32::MAX}else{a[i]};

    let max_left_b = if j == 0 {i32::MIN}else {a[j-1]};
    let min_right_b = if j == m {i32::MAX}else{a[j]};

    if max_left_a <= min_right_b && max_left_b <= min_right_a{
        if m+n%2 == 0 {
            return
            (max_left_a.max(max_left_b) as f64 + min_right_a.min(min_right_b) as f64)/2.0;
        }else{
            max_left_a.max(max_left_b) as f66
        }
    }else min_right_a < max_left_b {
        left +=1;
    }else{
       right -=1;
    }

   } 
    unreachable!()
    }
