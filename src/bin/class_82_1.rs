use std::cmp;

/**
 * 121. 买卖股票的最佳时机
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/description/
 */
fn main() {}

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![-1; n]; n];
        Self::f(&prices, 0, n - 1, &mut dp)
    }

    pub fn f(prices: &[i32], l: usize, r: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if l >= r {
            return 0;
        }
        if dp[l][r] != -1 {
            return dp[l][r];
        }

        let mut ans = prices[r] - prices[l];
        ans = cmp::max(ans, Self::f(prices, l + 1, r, dp));
        ans = cmp::max(ans, Self::f(prices, l, r - 1, dp));
        dp[l][r] = ans;
        ans
    }

    pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![0; n];

        for l in (0..n - 1).rev() {
            for r in l + 1..n {
                dp[r] = cmp::max(cmp::max(dp[r], prices[r] - prices[l]), dp[r - 1]);
            }
        }
        dp[n - 1]
    }

    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut min = prices[0];
        for i in 1..prices.len() {
            min = cmp::min(min, prices[i]);
            ans = cmp::max(ans, prices[i] - min);
        }
        ans
    }
}
