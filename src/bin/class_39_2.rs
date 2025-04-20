fn main() {
    let test = "3[a]4[bc]5[3[a]]".to_string();
    let ans = Solution::decode_string(test);
    println!("{ans}");
}

pub struct Solution {}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let c: Vec<char> = s.chars().collect();
        let mut i = 0;
        Self::helper(&c, &mut i)
    }

    pub fn helper(s: &[char], i: &mut usize) -> String {
        let mut path = String::new();
        let mut cnt = 0;
        while *i < s.len() && s[*i] != ']' {
            if s[*i].is_alphabetic() {
                path.push(s[*i]);
                *i += 1;
            } else if s[*i].is_numeric() {
                cnt = cnt * 10 + s[*i].to_digit(10).unwrap() as usize;
                *i += 1;
            } else {
                *i += 1;
                path.push_str(&Self::get(cnt, &Self::helper(s, i)));
                *i += 1;
                cnt = 0;
            }
        }
        path
    }

    pub fn get(cnt: usize, s: &str) -> String {
        std::iter::repeat_n(s, cnt).collect()
    }
}
