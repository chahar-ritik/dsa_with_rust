use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut set = HashSet::new();

        let mut left = 0;
        let mut max_len = 0;

        for right in 0..chars.len() {
            // shrink window until duplicate is removed
            while set.contains(&chars[right]) {
                set.remove(&chars[left]);
                left += 1;
            }

            set.insert(chars[right]);
            max_len = max_len.max(set.len());
        }

        max_len as i32
    }
}
