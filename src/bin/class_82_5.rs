use std::cmp;

/**
 * 714. 买卖股票的最佳时机含手续费
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/
 */
fn main() {}

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        // prepare[i] 0..i 已经支付过手续费和扣除一次买入价格了
        // done[i] 0..i 位置的利润最大值

        // done[i] = max(done[i-1], prepare[i-1] + prices[i])
        // prepare[i] = max(prepare[i-1], done[i] - fee - prices[i])

        let mut prepare = -fee - prices[0];
        let mut done = 0;

        for i in 1..prices.len() {
            done = cmp::max(done, prepare + prices[i]);
            prepare = cmp::max(prepare, done - fee - prices[i]);
        }
        done
    }
}
