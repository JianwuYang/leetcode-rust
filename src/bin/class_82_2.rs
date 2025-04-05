/**
 * 122. 买卖股票的最佳时机 II
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/description/
 */

 pub struct Solution {

 }

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..prices.len() {
            let profit = prices[i] - prices[i - 1];
            if profit > 0 {
                ans += profit;
            }
        }
        ans
    }
}

fn main() {}    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..prices.len() {
            let profit = prices[i] - prices[i - 1];
            if profit > 0 {
                ans += profit;
            }
        }
        ans
    }