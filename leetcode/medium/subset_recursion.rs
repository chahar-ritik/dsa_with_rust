impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn allsubset(nums: &Vec<i32>, i: usize, ans: &mut Vec<i32>, fans: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                fans.push(ans.clone());
                return;
            }

            // include
            ans.push(nums[i]);
            allsubset(nums, i + 1, ans, fans);

            // exclude
              ans.pop();
            allsubset(nums, i + 1, ans, fans);

            
          
        }

        let mut ans = Vec::new();
        let mut fans = Vec::new();

        allsubset(&nums, 0, &mut ans, &mut fans);

        fans
    }
}
