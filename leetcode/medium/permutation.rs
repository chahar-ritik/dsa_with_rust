impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn allpermutation(nums: &mut Vec<i32>,ans: &mut Vec<Vec<i32>> ,idx: usize ){

           if idx == nums.len() {
            ans.push(nums.clone());
            return;
          }


       let mut i = idx;
       while i < nums.len(){

         nums.swap(i, idx);
          allpermutation(nums,ans,idx+1);
            // backtrack
            nums.swap(i, idx); 
            i += 1;
          
       }
           
            
        }
          let mut ans: Vec<Vec<i32>> = Vec::new();
       allpermutation(&mut nums, &mut ans , 0 );

       ans
    }
}