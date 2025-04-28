use std::collections::HashMap;

fn main() {
    let ans = min_days(10);
    println!("{ans}")
}

pub fn min_days(n: i32) -> i32 {
    let mut dp = HashMap::new();

    fn helper(n: i32, dp: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 {
            return 0;
        }

        if n == 1 {
            return 1;
        }

        if dp.contains_key(&n) {
            return dp[&n];
        }

        let ans = (helper(n / 2, dp) + n % 2).min(helper(n / 3, dp) + n % 3) + 1;

        dp.insert(n, ans);

        ans
    }

    helper(n, &mut dp)
}
