use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return "".to_string();
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut need = HashMap::new();

        for c in t.chars() {
            *need.entry(c).or_insert(0) += 1;
        }

        let mut have = HashMap::new();
        let mut required = need.len();
        let mut formed = 0;

        let mut l = 0;
        let mut min_len = usize::MAX;
        let mut min_l = 0;

        for r in 0..s_chars.len() {
            let c = s_chars[r];
            *have.entry(c).or_insert(0) += 1;

            if need.contains_key(&c) && have[&c] == need[&c] {
                formed += 1;
            }

            while formed == required {
                if r - l + 1 < min_len {
                    min_len = r - l + 1;
                    min_l = l;
                }

                let left_char = s_chars[l];
                *have.get_mut(&left_char).unwrap() -= 1;

                if need.contains_key(&left_char) && have[&left_char] < need[&left_char] {
                    formed -= 1;
                }

                l += 1;
            }
        }

        if min_len == usize::MAX {
            "".to_string()
        } else {
            s_chars[min_l..min_l + min_len].iter().collect()
        }
    }
}
