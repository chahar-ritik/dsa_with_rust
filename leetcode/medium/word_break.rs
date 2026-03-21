use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
         let n = s.len();
    let s = s.as_bytes(); 
    let dict: HashSet<&[u8]> = word_dict.iter()
        .map(|w| w.as_bytes())
        .collect();

    let mut dp = vec![false; n + 1];
    dp[0] = true; 
    for i in 1..=n {
        for j in 0..i {
            
            if dp[j] && dict.contains(&s[j..i]) {
                dp[i] = true;
                break; 
            }
        }
    }

    dp[n]
    }
}g