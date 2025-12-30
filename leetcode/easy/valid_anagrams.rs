use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false; // ok for LeetCode ASCII
        }

        let mut map = HashMap::new();

        for ch in s.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            if let Some(v) = map.get_mut(&ch) {
                *v -= 1;
                if *v < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
