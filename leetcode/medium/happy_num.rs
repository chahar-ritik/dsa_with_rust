// Using floyd's cycle detection
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;

        loop {
            slow = Solution::digit_square_sum(slow);
            fast = Solution::digit_square_sum(
                Solution::digit_square_sum(fast)
            );

            if slow == fast {
                return slow == 1;
            }
        }
    }

    fn digit_square_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let d = n % 10;
            sum += d * d;
            n /= 10;
        }
        sum
    }
}

// this is logically correct but not floyd idiomatic
//If a cycle exists, slow and fast must meet for idiomatic
//and here two condition if true than we get 1 on both slow after some values when fast get 1 or a cycle where fast == slow but that value is not 1 
/*impl Solution {
    pub fn is_happy(n: i32) -> bool {
        if n == 1{
            return true;
        }
        let mut slow = n;
        let mut fast = n;
        
        while fast != 1 {
            slow = Solution::digitsquaresum(slow);
            fast = Solution::digitsquaresum(Solution::digitsquaresum(fast));

            if slow == fast {
                if slow != 1
                {
                    return false;
                }
                else{
                    return true;
                }
            }

        }
       return true;
        
    }

    fn digitsquaresum(mut n : i32) -> i32{
        let mut sum= 0;
        while n > 0 {
            let dig = n%10;
            sum += dig*dig;
            n = n/10;
        } 
        sum
    }
} */