impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {

        fn is_palindrome(s: &str) -> bool {
            let bytes = s.as_bytes();
            let mut l = 0;
            let mut r = bytes.len().saturating_sub(1);

            while l < r {
                if bytes[l] != bytes[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }


        fn pal_partition(s: &str, partition: &mut Vec<String> , ans: &mut Vec<Vec<String>>){
            if s.is_empty(){
                ans.push(partition.clone());
                return;
            }
        
             let n = s.len();
        for i in 0..n{
            let part = &s[0..i+1];
            if is_palindrome(part){
                partition.push(part.to_string());
                pal_partition(&s[i+1..n], partition , ans);
                partition.pop();

            }

        }

        }
        let mut partition: Vec<String> = Vec::new();
        let mut ans: Vec<Vec<String>> = Vec::new();
        pal_partition(&s , &mut partition , &mut ans);

        ans
    }
}