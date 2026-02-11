impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 || s.len() <= num_rows {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut curr_row = 0usize;
        let mut going_down = false;

        for ch in s.chars() {
            rows[curr_row].push(ch);

            if curr_row == 0 || curr_row == num_rows - 1 {
                going_down = !going_down;
            }

            curr_row = if going_down {
                curr_row + 1
            } else {
                curr_row - 1
            };
        }

        rows.concat()
    }
}
