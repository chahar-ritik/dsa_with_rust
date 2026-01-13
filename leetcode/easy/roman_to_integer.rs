impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            let value = match chars[i] {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if i + 1 < chars.len() {
                let nextvalue = match chars[i + 1] {
                    'I' => 1,
                    'V' => 5,
                    'X' => 10,
                    'L' => 50,
                    'C' => 100,
                    'D' => 500,
                    'M' => 1000,
                    _ => 0,
                };

                if value < nextvalue {
                    total -= value;
                } else {
                    total += value;
                }
            } else {
                total += value;
            }
        }

        total
    }
}
