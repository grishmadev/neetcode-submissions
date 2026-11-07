impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let (mut l, mut r) = (0, 1);
        while r < prices.len() {
            let profit = prices[r] - prices[l];
            if profit > 0 {
                max_profit = max_profit.max(profit);
            } else {
                l = r;
            }
            r += 1;
        }
        max_profit
    }
}
