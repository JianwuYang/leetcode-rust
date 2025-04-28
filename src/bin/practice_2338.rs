fn main() {
    todo!();
}

const MOD: i32 = 1000000007;

pub struct Solution {}

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![-1; max_value as usize]; n];

        Self::helper(&n, &max_value, 0, 1, &mut dp)
    }

    pub fn helper(n: &usize, max_value: &i32, i: usize, pre: i32, dp: &mut [Vec<i32>]) -> i32 {
        if i == *n {
            return 1;
        }
        if pre == *max_value {
            return 1;
        }

        if dp[i][pre as usize] != -1 {
            return dp[i][pre as usize];
        }

        let mut ans = 0;

        for cur in pre..=*max_value {
            if cur % pre == 0 {
                ans = (ans + Self::helper(n, max_value, i + 1, cur, dp)) % MOD;
            }
        }

        dp[i][pre as usize] = ans;

        ans
    }
}
