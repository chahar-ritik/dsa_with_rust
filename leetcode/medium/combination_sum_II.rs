use std::collections::HashSet;
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    

        fn combination(candidates: &Vec<i32>, target: i32 ,  i: usize ,comb: &mut Vec<i32>, set : &mut HashSet<Vec<i32>> , ans: &mut Vec<Vec<i32>>){
           
            if target == 0{
                
                if !(set.contains(comb)){
                set.insert(comb.clone());
                ans.push(comb.clone());
                
                }
                return;
            }

             if i == candidates.len() || target < 0 {
                return;
            }

            comb.push(candidates[i]);
            combination(candidates , target-candidates[i] , i+1 , comb , set , ans);
            comb.pop();
            combination(candidates , target , i+1 , comb , set , ans );
        }

        let mut set: HashSet<Vec<i32>> = HashSet::new();
        let mut ans : Vec<Vec<i32>> = Vec::new();
        let mut comb : Vec<i32> = Vec::new();
        candidates.sort();

        combination(&candidates , target , 0 , &mut comb , &mut set , &mut ans);
        ans
    }
}
//Optimal approach

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    

        fn combination(candidates: &Vec<i32>, target: i32 ,  start : usize ,comb: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>){
           
            if target == 0{
                ans.push(comb.clone());
                return;
                }
            
             for i in start..candidates.len(){
              if i>start && candidates[i] == candidates[i+1]{
                continue;
              }
              if candidates[i] > target{
                break;
              }

              comb.push(candidates[i]);
            combination(candidates , target-candidates[i] , i+1 , comb  , ans);
            comb.pop();
             }
        }

      
        let mut comb : Vec<i32> = Vec::new();
         let mut ans : Vec<Vec<i32>> = Vec::new();
        candidates.sort();

        combination(&candidates , target , 0 , &mut comb , &mut ans);
        ans
    }
}