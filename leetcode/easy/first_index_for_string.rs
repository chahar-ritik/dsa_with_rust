impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let h = haystack.as_bytes();
        let n = needle.as_bytes();

        if n.len() > h.len() {
            return -1;
        }

        for i in 0..=h.len() - n.len() {
            if &h[i..i + n.len()] == n {
                return i as i32;
            }
        }

        -1
    }
}
