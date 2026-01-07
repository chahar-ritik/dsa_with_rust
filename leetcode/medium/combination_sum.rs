use std::collections::HashSet;


impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

       fn combination(candidates: &Vec<i32> , mut target: i32 , i: usize , comb: &mut Vec<i32> , ans : &mut Vec<Vec<i32>>,set: &mut HashSet<Vec<i32>>){
            if i == candidates.len() || target < 0{
                return;
            }
            if target == 0  {
                if !(set.contains(comb)){
                set.insert(comb.clone()); 
                ans.push(comb.clone());  
                }
                return;
            }

         
            comb.push(candidates[i]);
            // single time include any number
            combination(candidates, target - candidates[i],i+1 , comb , ans,set);
            // multiple time included
             combination(candidates, target - candidates[i],i , comb , ans,set);
             // exclude form combination
               comb.pop();
              combination(candidates, target ,i+1 , comb , ans,set);
            



        }

        let mut comb : Vec<i32> = Vec::new();
        let mut ans : Vec<Vec<i32>> = Vec::new();
        let mut set: HashSet<Vec<i32>> = HashSet::new();
         combination(&candidates , target , 0 , &mut comb , &mut ans,&mut set);
         
          ans
    }
}