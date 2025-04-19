
use crate::common::Solution;

use std::collections::HashSet;
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        fn collect(nums: &mut Vec<i32>, i: usize, ans: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                ans.push(nums.clone());
            } else {
                let mut set = HashSet::new();

                for j in i..nums.len() {
                    if !set.contains(&nums[j]) {
                        nums.swap(i, j);
                        collect(nums, i + 1, ans);
                        nums.swap(i, j);
                        set.insert(nums[j]);
                    }
                }
            }
        }

        collect(&mut nums, 0, &mut ans);

        ans
    }
}
