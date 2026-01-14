impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
      let mut k = 0;
      for i in 0..nums.len(){
        if nums[i] != val {
         nums[k] = nums[i];
         k+=1;
        }
      }
      k as i32
    }
}
// one method that we can use is retain that rust gives us to only retain the value that are true for certain condtion
/*
impl Solution {
 pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
 nums.retain(|&ele| ele != val );
  nums.len() as i32 
  }
  } */