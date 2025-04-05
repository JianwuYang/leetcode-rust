use std::cmp;

/**
 * 309. 买卖股票的最佳时机含冷冻期
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-cooldown/description/
 */

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // prepare[i] 0..i 范围上满足冷冻期需求且已经扣除过买入价格的最大
        // done[i] 0..i 范围上 执行交易的最大收益

        // done[i] = max(done[i - 1], prepare[i - 1] + prices[i])
        // prepare[i] = max(prepare[i - 1], done[i - 2] - prices[i])

        let n = prices.len();

        if n < 2 {
            return 0;
        }

        let mut prepare = vec![0; n];
        let mut done = vec![0; n];

        prepare[1] = cmp::max(-prices[0], -prices[1]);
        done[1] = cmp::max(0, prices[1] - prices[0]);

        for i in 2..n {
            done[i] = cmp::max(done[i - 1], prepare[i - 1] + prices[i]);
            prepare[i] = cmp::max(prepare[i - 1], done[i - 2] - prices[i]);
        }

        done[n - 1]
    }

    // 空间压缩
    pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
        let n = prices.len();

        if n < 2 {
            return 0;
        }

        let mut prepare = (-prices[0]).max(-prices[1]);
        let mut done1 = 0.max(prices[1] - prices[0]);
        let mut done2 = 0;

        for i in 2..n {
            prepare = prepare.max(done2 - prices[i]);
            done2 = done1;
            done1 = done1.max(prepare + prices[i]);
        }

        done1
    }
}

fn main() {}
