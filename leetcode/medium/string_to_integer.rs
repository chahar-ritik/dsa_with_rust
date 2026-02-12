pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut i = 0;

       
        while i < n && bytes[i] == b' ' {
            i += 1;
        }

        
        let mut sign = 1;
        if i < n && (bytes[i] == b'+' || bytes[i] == b'-') {
            if bytes[i] == b'-' {
                sign = -1;
            }
            i += 1;
        }

       
        let mut result: i32 = 0;

        while i < n && bytes[i].is_ascii_digit() {
            let digit = (bytes[i] - b'0') as i32;

          
            if result > (i32::MAX - digit) / 10 {
                return if sign == 1 {
                    i32::MAX
                } else {
                    i32::MIN
                };
            }

            result = result * 10 + digit;
            i += 1;
        }

        result * sign
    }