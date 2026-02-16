impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let map = vec![
            "",     // 0
            "",     // 1
            "abc",  // 2
            "def",  // 3
            "ghi",  // 4
            "jkl",  // 5
            "mno",  // 6
            "pqrs", // 7
            "tuv",  // 8
            "wxyz", // 9
        ];

        let mut result = Vec::new();
        let mut path = String::new();
        Self::backtrack(0, &digits, &map, &mut path, &mut result);
        result
    }

    fn backtrack(
        index: usize,
        digits: &String,
        map: &Vec<&str>,
        path: &mut String,
        result: &mut Vec<String>,
    ) {
        if index == digits.len() {
            result.push(path.clone());
            return;
        }

        let digit = digits.as_bytes()[index] - b'0';
        let letters = map[digit as usize];

        for ch in letters.chars() {
            path.push(ch);
            Self::backtrack(index + 1, digits, map, path, result);
            path.pop(); 
        }
    }
}
