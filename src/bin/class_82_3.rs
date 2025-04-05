use std::cmp;

/**
 * 123. 买卖股票的最佳时机 III
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iii/description/
 */
pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();

        let mut left = vec![0; n];
        let mut min = prices[0];
        for i in 1..n {
            min = cmp::min(min, prices[i]);
            left[i] = cmp::max(left[i - 1], prices[i] - min);
        }

        let mut right = vec![0; n];
        let mut max = prices[n - 1];
        for i in (0..n - 1).rev() {
            max = cmp::max(max, prices[i]);
            right[i] = cmp::max(right[i + 1], max - prices[i]);
        }
        let mut ans = 0;

        for i in 0..n - 1 {
            ans = cmp::max(ans, left[i] + right[i + 1]);
        }
        ans = cmp::max(ans, left[n - 1]);
        ans
    }

    pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
        let n = prices.len();

        // 0..i 范围上，可能发生交易的最大收益
        let mut dp1 = vec![0; n];
        let mut min = prices[0];
        for i in 1..n {
            min = cmp::min(min, prices[i]);
            dp1[i] = cmp::max(dp1[i - 1], prices[i] - min);
        }

        // 第二次交易以 i 作为卖出时间点的最大收益
        let mut dp2 = vec![0; n];

        for i in 1..n {
            for j in 0..i {
                dp2[i] = cmp::max(dp2[i], dp1[j] + prices[i] - prices[j]);
            }
        }
        let mut ans = 0;
        for i in 1..n {
            ans = cmp::max(ans, dp2[i]);
        }
        ans
    }

    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let n = prices.len();

        // 0..i 范围上，可能发生交易的最大收益
        let mut dp1 = vec![0; n];
        let mut min = prices[0];
        for i in 1..n {
            min = cmp::min(min, prices[i]);
            dp1[i] = cmp::max(dp1[i - 1], prices[i] - min);
        }

        let mut best = vec![0; n];
        best[0] = -prices[0];
        for i in 1..n {
            best[i] = cmp::max(best[i - 1], dp1[i] - prices[i]);
        }

        let mut dp2 = vec![0; n];
        for i in 1..n {
            dp2[i] = best[i] + prices[i];
        }

        let mut ans = 0;
        for i in 1..n {
            ans = cmp::max(ans, dp2[i]);
        }
        ans
    }

    pub fn max_profit_v3(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp1 = vec![0; n];
        let mut min = prices[0];
        let mut best = vec![0; n];
        best[0] = -prices[0];
        let mut dp2 = vec![0; n];
        let mut ans = 0;
        for i in 1..n {
            min = cmp::min(min, prices[i]);
            dp1[i] = cmp::max(dp1[i - 1], prices[i] - min);
            best[i] = cmp::max(best[i - 1], dp1[i] - prices[i]);
            dp2[i] = best[i] + prices[i];
            ans = cmp::max(ans, dp2[i]);
        }
        ans
    }

    pub fn max_profit_v4(prices: Vec<i32>) -> i32 {
        let mut dp1 = 0;
        let mut min = prices[0];
        let mut best = -prices[0];
        let mut ans = 0;
        for price in prices {
            min = cmp::min(min, price);
            dp1 = cmp::max(dp1, price - min);
            best = cmp::max(best, dp1 - price);
            ans = cmp::max(ans, best + price);
        }
        ans
    }
}

fn main() {}
