impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
       let mut stack = Vec::new();
       let n = temperatures.len();
       let mut ans = vec![0;n];
      for temp in 0..n{
          let curr_temp = temperatures[temp];
          while let Some(&top) = stack.last(){
            if curr_temp > temperatures[top] {
                stack.pop();

                ans[top] = temp as i32-top as i32;

            }
            else{
                break;
            }
          }
          stack.push(temp)
       }

       ans
    }
}