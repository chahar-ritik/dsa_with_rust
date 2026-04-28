// how columns are in excel named or labeled as A , B ... AA , AB ...
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0;

        for ch in column_title.chars() {
            let value = (ch as u8 - b'A' + 1) as i32;
            result = result * 26 + value;
        }

        result
    }
}