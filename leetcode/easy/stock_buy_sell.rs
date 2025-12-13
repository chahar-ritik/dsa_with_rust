
// best approach as here we two pointer l and r l will be updated on certain condition and r every time

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, 1);
        let mut res = 0;

        while r < prices.len() {
            if prices[l] < prices[r] {
                let profit = prices[r] - prices[l];
                res = res.max(profit);
            } else {
                l = r;
            }
            r += 1;
        }

        res
    }
}