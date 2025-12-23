//Input: s = "the sky is blue"
//Output: "blue is sky the"


impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ans: Vec<char> = s.chars().collect();
        ans.reverse(); 
        let mut i = 0;
        let  n = ans.len();
        
        while i < n {
            if ans[i] == ' '{
                i+=1;
                continue;
            }
            let mut end = i;
            while end + 1 < n && ans[end+1] != ' '{
                end += 1;
            }

            ans[i..=end].reverse();
            i = end + 1;

        }

        let mut write = 0;
        let mut i = 0;

        while i < n && ans[i] == ' '{
            i +=1;
        }
   

        while i < n{


            while i < n && ans[i] != ' '{
                ans[write] = ans[i];
                write +=1;
                i +=1;
            }

            while i < n && ans[i] == ' ' {
                i += 1;
            }

            if i < n {
                ans[write] = ' ';
                write +=1;
            }
        }

        ans.truncate(write);

        ans.iter().collect()

       
    }
}