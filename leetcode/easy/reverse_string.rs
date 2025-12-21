impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
      let mut i = 0;
      let mut j = s.len()-1;
      // indtead of s.len()-1; we can use .saturating_sub(1) that will safely -1 if given usize is above zero else return zero that will not panic the code
      
         while i<j {
            s.swap(i,j);
            i+=1;
            j-=1;
         }
    }
}