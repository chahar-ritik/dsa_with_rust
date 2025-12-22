impl Solution {
    pub fn remove_occurrences(mut s: String, part: String) -> String {

        if part.is_empty() {
        return s;
    }
  while let Some(pos) = s.find(&part) {
    s.replace_range(pos..pos+part.len(),"");
  }
  s
    }
}
/* withod methods
pub fn remove_occurrences(s: String, part: String) -> String {
    let part_chars: Vec<char> = part.chars().collect();
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        stack.push(c);

        if stack.len() >= part_chars.len() {
            let start = stack.len() - part_chars.len();
            if stack[start..] == part_chars[..] {
                for _ in 0..part_chars.len() {
                    stack.pop();
                }
            }
        }
    }

    stack.iter().collect()
}
 */