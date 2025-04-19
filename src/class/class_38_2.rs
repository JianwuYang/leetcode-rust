use crate::common::Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        nums.sort();

        fn collect(nums: &Vec<i32>, start: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            res.push(path.clone());
            for i in start..nums.len() {
                if i > start && nums[i] == nums[i - 1] {
                    continue;
                }

                path.push(nums[i]);
                collect(nums, i + 1, path, res);
                path.pop();
            }
        }

        collect(&nums, 0, &mut path, &mut res);

        res
    }
}
