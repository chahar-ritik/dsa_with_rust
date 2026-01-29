// it is not a sliding window pattern 
// it is based on prefix sum that we stored in hashmap and when next time we will see this sum we will calculate length 
//(sum , idx)
use std::collections::HashMap;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        map.insert(0,-1);
        let mut ans = 0;
         let mut sum = 0;
        for (i,num) in nums.iter().enumerate(){
            if *num == 0{
                sum-=1;
            }
            else{
                sum+=1;
            }

            if let Some(&idx) = map.get(&sum){
               ans = ans.max(i as i32 - idx);
            }
            else{
                map.insert(sum , i as i32);

            }

        }
       ans  
    }
}