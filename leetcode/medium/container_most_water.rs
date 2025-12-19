 
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
     let mut l = 0 as usize;
     let mut r = height.len()-1;
     let mut  mw = 0;
     let mut cw = 0;
     while ( l < r){
        if height[l] < height[r]{
            cw = height[l] * (r-l)as i32;
            l += 1;
            if mw < cw {
                mw = cw;
            }
        }
        else{
            cw = height[r] * (r-l)as i32;
            r -= 1;
            if mw < cw {
                mw = cw;
            }
        }

     }
      return mw
    }
}