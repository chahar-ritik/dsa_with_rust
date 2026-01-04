impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
          nums.sort();
        fn allsubset(nums: &Vec<i32>, ans: &mut Vec<i32> , fans: &mut Vec<Vec<i32>> , i:usize){

            if i == nums.len(){
                fans.push(ans.clone());
                return;
            }
             
             //include
            ans.push(nums[i]);
            allsubset(nums, ans , fans , i+1);

            //exclude
            ans.pop();
            //after sorting skip if we have same element as previous
            let mut idx:usize = i+1;
            while idx < nums.len() && nums[idx] == nums[idx-1]{
                idx+=1;
            }
            allsubset(nums, ans , fans , idx);

        }
     let mut ans: Vec<i32> = Vec::new();
     let mut fans: Vec<Vec<i32>> = Vec::new();

     allsubset(&nums ,&mut ans,&mut fans ,0);

     fans

    }
}