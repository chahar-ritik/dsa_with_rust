
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
      let n = s1.len();
      let m = s2.len();

      if n > m {
        return false;
      }

      let mut freq1 = vec![0;26];
      let mut freq2 = vec![0;26];
         
         let s1 = s1.as_bytes();
          let s2 = s2.as_bytes();

       for i in 0..n {
          freq1[(s1[i]- b'a') as usize] += 1;
          freq2[(s2[i]- b'a') as usize] += 1;

       }

       if freq1 == freq2 {
        return true;
       }

       // only one vector is used and only single loop 
       // remove one from from first index and add one to next in s2

       for i in n..m {
        freq2[(s2[i] - b'a') as usize] +=1;

        freq2[(s2[i-n] - b'a') as usize] -=1;

        if freq1 == freq2 {
            return true;
        }
       }

       return false;


    }
}