use crate::common::Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        fn collect(nums: &mut Vec<i32>, i: usize, ans: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                ans.push(nums.clone());
            } else {
                for j in i..nums.len() {
                    nums.swap(i, j);
                    collect(nums, i + 1, ans);
                    nums.swap(i, j);
                }
            }
        }

        collect(&mut nums, 0, &mut ans);

        ans
    }
}
