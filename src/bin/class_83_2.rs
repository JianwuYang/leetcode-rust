/**
 * 629. K 个逆序对数组
 * https://leetcode.cn/problems/k-inverse-pairs-array/description/
 */
fn main() {
    Solution::k_inverse_pairs(3, 0);
}

pub struct Solution {}

const MOD: i32 = 1000000007;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        // dp[i][j] 0...i 个数存在 j 个逆序对的排列数
        // 结果应为 dp[n][k]
        let (n, k) = (n as usize, k as usize);
        let mut dp = vec![vec![0; k + 1]; n + 1];

        // dp[i][j] = 如果第 i 个数放在最后，那么新增0个逆序对，需要前 i-1 个数搞定 j 个逆序对，这种情况就是 dp[i - 1][j]
        // 类似的 如果第 i 个数放到最后一个数之前， 会新增 1 个逆序对，此时需要前 i - 1 个数搞定 j - 1 个逆序对 dp[i - 1][j - 1]
        // 举例 dp[4][3]  a b c    d
        // ->  a b c     a b d c    a d b c     d a b c
        // -》 dp[3][3] + dp[3][2] + dp[3][1] + dp[3][0]
        // i > j => 0 ~ j - 1
        // 如果 dp[4][5] a b c   d
        //
        // -> dp[3][5] + dp[3][4] + dp[3][3] + dp[3][2]
        // i <= j => j - i + 1 ~ j - 1

        // 基本条件 dp[0][0] 前 0 个数搞定 0个逆序对，排列为 1 dp[0][j > 0] 均为 0
        // dp[i][0] = 1
        dp[0][0] = 1;

        for i in 1..=n {
            dp[i][0] = 1;

            for j in 1..=k {
                if i > j {
                    for p in 0..=j {
                        dp[i][j] = (dp[i][j] + dp[i - 1][p]) % MOD;
                    }
                } else {
                    for p in j - i + 1..=j {
                        dp[i][j] = (dp[i][j] + dp[i - 1][p]) % MOD;
                    }
                }
            }
        }
        dp[n][k]
    }

    // 优化枚举
    pub fn k_inverse_pairs_v1(n: i32, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        let mut dp = vec![vec![0; k + 1]; n + 1];

        dp[0][0] = 1;

        for i in 1..=n {
            dp[i][0] = 1;

            // 考虑到上面情况的分析，实际上是维护了一个 j - i + 1 ~ j 的一个窗口
            let mut window = 1;
            for j in 1..=k {
                if i > j {
                    window = (window + dp[i - 1][j]) % MOD;
                } else {
                    window = ((window + dp[i - 1][j]) % MOD - dp[i - 1][j - i] + MOD) % MOD;
                }
                dp[i][j] = window;
            }
        }
        dp[n][k]
    }
}
