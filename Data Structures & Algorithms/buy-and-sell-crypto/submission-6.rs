use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut l = 0;
        let mut r = 1;

        while r < prices.len()
        {
            if prices[l]>=prices[r]
            {
                l = r;
                r += 1;
            }
            else
            {
                profit = cmp::max(profit,prices[r] - prices[l]);
                r += 1;
            }
        }
        profit
    }
}
/*
while r < prices.len()

if prices[l]>prices[r]
    l = r
    r = r + 1
prices[l] < prices[r]
    update profit
    r = r + 1
*/