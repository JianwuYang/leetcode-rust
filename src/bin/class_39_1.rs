fn main() {
    let ans = Solution::calculate("6-4/2".to_string());
    println!("{ans}");
}

pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let c = s.chars().collect();
        Self::f(&c, 0).0
    }

    pub fn f(s: &Vec<char>, mut i: usize) -> (i32, usize) {
        let mut cur = 0;
        let mut numbers: Vec<i32> = Vec::new();
        let mut ops: Vec<char> = Vec::new();
        let mut pos;
        while i < s.len() && s[i] != ')' {
            if let Some(digit) = s[i].to_digit(10) {
                cur = cur * 10 + digit as i32;
                i += 1;
            } else if s[i] != '(' {
                Self::push(&mut numbers, &mut ops, cur, s[i]);
                i += 1;
                cur = 0;
            } else {
                (cur, pos) = Self::f(s, i + 1);
                i = pos + 1;
            }
        }
        Self::push(&mut numbers, &mut ops, cur, '+');
        (Self::compute(&numbers, &ops), i)
    }

    pub fn push(numbers: &mut Vec<i32>, ops: &mut Vec<char>, cur: i32, op: char) {
        let n = numbers.len();
        if n == 0 || ops[n - 1] == '+' || ops[n - 1] == '-' {
            numbers.push(cur);
            ops.push(op);
        } else {
            let top_number = numbers[n - 1];
            let top_op = ops[n - 1];
            if top_op == '*' {
                numbers[n - 1] = top_number * cur;
            } else {
                numbers[n - 1] = top_number / cur;
            }
            ops[n - 1] = op;
        }
    }

    pub fn compute(numbers: &[i32], ops: &[char]) -> i32 {
        let n = numbers.len();
        let mut ans = numbers[0];
        for i in 1..n {
            if ops[i - 1] == '+' {
                ans += numbers[i];
            } else {
                ans -= numbers[i];
            }
        }
        ans
    }
}
