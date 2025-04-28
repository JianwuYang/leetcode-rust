fn main() {
    todo!();
}

pub struct Solution {}

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let (n, a, b) = (n as i64, a as i64, b as i64);
        let lcm = Self::lcm(a, b);
        let mut ans = 0;

        let mut l = 0;
        let mut r = n * a.min(b);

        while l <= r {
            let m = l + ((r - l) >> 1);
            if m / a + m / b - m / lcm >= n {
                ans = m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        (ans % 1000000007) as i32
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    fn lcm(a: i64, b: i64) -> i64 {
        a / Self::gcd(a, b) * b
    }
}
