impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let (mut l, mut r) = (0,0);
        while l < prices.len() {
            let cur_profit = prices[r] - prices[l];
            if cur_profit <= max_profit {
                r += 1;
            } else {
                max_profit = cur_profit;
            }
            if r >= prices.len() {
                l += 1;
                r = l;
            }
        }
        max_profit
    }
}
