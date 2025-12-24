impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() < 2 {
            return chars.len() as i32;
        }

        let mut write = 0;
        let mut count = 1;

        for i in 1..=chars.len() {
            if i < chars.len() && chars[i] == chars[i - 1] {
                count += 1;
            } else {
              
                chars[write] = chars[i - 1];
                write += 1;

               
                if count > 1 {
                    for ch in count.to_string().chars() {
                        chars[write] = ch;
                        write += 1;
                    }
                }

                count = 1;
            }
        }

        write as i32
    }
}
