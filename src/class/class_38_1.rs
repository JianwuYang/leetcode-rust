use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     *
     * @param s string字符串
     * @return string字符串一维数组
     */
    pub fn generatePermutation(&self, s: String) -> Vec<String> {
        let c: Vec<char> = s.chars().collect();
        let mut path = Vec::new();
        let mut ans = HashSet::new();
        Self::f(&c, 0, &mut path, &mut ans);
        ans.into_iter().collect()
    }

    fn f(s: &Vec<char>, i: usize, path: &mut Vec<char>, ans: &mut HashSet<String>) {
        if i == s.len() {
            ans.insert(path.iter().collect());
            return;
        }

        Self::f(s, i + 1, path, ans);

        path.push(s[i]);

        Self::f(s, i + 1, path, ans);

        path.pop();
    }
}
