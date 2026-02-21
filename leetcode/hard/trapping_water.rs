 //Most optimal approach
 
 pub fn trap(height: Vec<i32>) -> i32 {
    
        let mut ans = 0;
         let mut l = 0;
         let mut r = height.len()-1;
         let mut leftmax = 0;
         let mut rightmax = 0;

        while l < r {
            leftmax = leftmax.max(height[l]);
            rightmax = rightmax.max(height[r]);
             if leftmax < rightmax{
                ans+= leftmax - height[l];
                l+=1;
             }else{
                ans+= rightmax - height[r];
                r-=1;
             }
        }
        ans
    }


//prefix and suffix max approach

pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut ans = 0;
    let mut prefix_max = vec![0;n];
       prefix_max[0] = height[0];
    for i in 1..n{
        prefix_max[i] = prefix_max[i-1].max(height[i]);

    }
    let mut suffix_max = height[n-1];
    for i in (0..n).rev(){
        suffix_max = suffix_max.max(height[i]);
        ans+= prefix_max[i].min(suffix_max) - height[i];
    }

    ans
}


