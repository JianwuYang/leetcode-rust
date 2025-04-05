/**
 * 188. 买卖股票的最佳时机 IV
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iv/description/
 */
fn main() {}

use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let k = k as usize;

        // dp[i][j] 0..j 范围上发生了 i 次交易的最大利润
        // dp[i][j] = dp[i][j - 1] 当前股票不参与交易，0..j-1 已经完成了 i 次交易
        //          = dp[i-1][p] + prices[j] - prices[p] 在 0..p 上完成了 i-1次交易，然后p买进 j卖出 完成最后一次交易
        let mut dp = vec![vec![0; n]; k + 1];

        // dp[0][j] = 0 在任何范围上发生0次交易的收益都是0
        // dp[i][0] = 0 在 0..0 上无论发生多少次交易的收益都是0

        for i in 1..k + 1 {
            for j in 1..n {
                dp[i][j] = dp[i][j - 1];
                for p in 0..j {
                    dp[i][j] = cmp::max(dp[i][j], dp[i - 1][p] + prices[j] - prices[p]);
                }
            }
        }

        dp[k][n - 1]
    }

    // 优化枚举行为
    pub fn max_profit_v1(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let k = k as usize;

        let mut dp = vec![vec![0; n]; k + 1];

        for i in 1..k + 1 {
            let mut best = dp[i - 1][0] - prices[0];
            for j in 1..n {
                dp[i][j] = cmp::max(dp[i][j - 1], best + prices[j]);
                best = cmp::max(best, dp[i - 1][j] - prices[j]);
            }
        }

        dp[k][n - 1]
    }

    // 空间压缩
    pub fn max_profit_v2(k: i32, prices: Vec<i32>) -> i32 {
        let n: usize = prices.len();
        let k = k as usize;

        let mut dp: Vec<i32> = vec![0; n];

        for _ in 1..k + 1 {
            let mut best = dp[0] - prices[0];
            for j in 1..n {
                let cur = cmp::max(dp[j - 1], best + prices[j]);
                best = cmp::max(best, dp[j] - prices[j]);
                dp[j] = cur;
            }
        }

        dp[n - 1]
    }
}
