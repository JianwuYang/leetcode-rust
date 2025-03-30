fn main() {}

// 2109. 向字符串添加空格
// https://leetcode.cn/problems/adding-spaces-to-a-string/description/?envType=daily-question&envId=2025-03-30
pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let mut ans = String::new();

    let mut idx = 0;

    for (i, ch) in s.chars().enumerate() {
        while idx < spaces.len() && i == spaces[idx] as usize {
            ans.push(' ');
            idx += 1;
        }

        ans.push(ch);
    }

    ans
}
