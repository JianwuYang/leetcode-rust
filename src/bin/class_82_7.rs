pub struct Solution {}

const MOD: i32 = 1000000007;

impl Solution {
    // 一个错误的尝试
    // 利用 status 记录 数字被使用的状态 p 为上一个数字
    // 第i个位置，f(i, p, status)，可以尝试 大于p的所有可用数字或者小于p的所有可用数字
    // 但是 n 最大值为 200，明显会超出时间复杂度

    // 正确尝试，记录小于前一个数字的数字数量 less，那么大于前一个数字的数字数量 n - less - i
    // f(i, less, n)，那么对于 i D 的话可以选择 第x 个小于 less的数字类比 I
    pub fn num_perms_di_sequence(str: String) -> i32 {
        let n = str.len();
        let s: Vec<char> = str.chars().collect();
        Self::f(&s, 0, n + 1, n + 1)
    }

    pub fn f(s: &[char], i: usize, less: usize, n: usize) -> i32 {
        if i == n {
            return 1;
        }
        let mut ans = 0;

        if i == 0 || s[i - 1] == 'D' {
            // 选择一个更小的数 可选范围 0 .. less-1
            for next_less in 0..less {
                ans += Self::f(s, i + 1, next_less, n);
            }
        } else {
            // 选择一个更大的数 可选的范围是 1 .. n - less - i
            for k in 1..=n - less - i {
                ans += Self::f(s, i + 1, less + k - 1, n);
            }
        }
        ans
    }

    pub fn num_perms_di_sequence_v1(str: String) -> i32 {
        // 数字的数量
        let n = str.len() + 1;

        // 字符串转换成字符数组
        let s: Vec<char> = str.chars().collect();

        let mut dp = vec![vec![0; n + 1]; n + 1];

        // 基本情况，i == n 时，满足条件数量为1
        for less in 0..n + 1 {
            dp[n][less] = 1;
        }

        // 遍历
        for i in (0..n).rev() {
            for less in 0..n + 1 {
                if i == 0 || s[i - 1] == 'D' {
                    for k in 0..less {
                        dp[i][less] = (dp[i][less] + dp[i + 1][k]) % MOD;
                    }
                } else {
                    let mut k = 1;
                    while k <= n.saturating_sub(less + i) {
                        dp[i][less] = (dp[i][less] + dp[i + 1][less + k - 1]) % MOD;
                        k += 1;
                    }
                }
            }
        }

        dp[0][n]
    }

    // 优化枚举
    pub fn num_perms_di_sequence_v2(str: String) -> i32 {
        // 数字的数量
        let n = str.len() + 1;

        // 字符串转换成字符数组
        let s: Vec<char> = str.chars().collect();

        let mut dp = vec![vec![0; n + 1]; n + 1];

        // 基本情况，i == n 时，满足条件数量为1
        for less in 0..n + 1 {
            dp[n][less] = 1;
        }

        // 遍历
        for i in (0..n).rev() {
            if i == 0 || s[i - 1] == 'D' {
                dp[i][1] = dp[i + 1][0];
                for less in 2..n + 1 {
                    dp[i][less] = (dp[i][less - 1] + dp[i + 1][less - 1]) % MOD;
                }
            } else {
                dp[i][n - i - 1] = dp[i + 1][n - i - 1];

                let mut less = (n - i) as i32 - 2;

                while less >= 0 {
                    {
                        let less = less as usize;
                        dp[i][less] = (dp[i][less + 1] + dp[i + 1][less]) % MOD;
                    }
                    less -= 1;
                }
            }
        }

        dp[0][n]
    }
}

fn main() {
    let ans = Solution::num_perms_di_sequence_v2("I".to_string());
    println!("{ans}");
}
