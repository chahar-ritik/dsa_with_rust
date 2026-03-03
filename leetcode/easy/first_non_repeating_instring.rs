//using queue more optimal solution posible with freq vector but overall tc same 
use std::collections::VecDeque;
use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut queue = VecDeque::new();
        let mut freq = HashMap::new();

      let byte = s.as_bytes();

      for i in 0..byte.len(){
            *freq.entry(byte[i]).or_insert(0)+=1;
            if *freq.get(&byte[i]).unwrap() == 1{
                queue.push_back(i);

            }
           
      }

      while let Some(&idx) = queue.front(){
         if *freq.get(&byte[idx]).unwrap() > 1 {
                    queue.pop_front();
                }else{
                    return idx as i32;
                }
      }
      -1
    }
}