impl Solution {
    // important thing this code will fail on leetcode as they check for edge case
    // and n will store -2^31 to 2^31-1 so last negative integer can't be stored as -(-n) 
    // as positve value one less than negative so we need to change i32 to i64 to solve this
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {

        // important change

        let mut n : i64 = n as i64;
        if n == 0 {
           return 1_f64
        }
        else if n < 0{
            x = 1_f64/x;
            n = -n;
        }
       let mut ans = 1_f64;
        while n > 0{
           if n%2 == 1 {
                ans *= x;
            }
            x *= x;
            n /=2;
        }

        return ans
    }
}