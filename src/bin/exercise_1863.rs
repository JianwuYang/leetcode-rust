/**
 * 1863. 找出所有子集的异或总和再求和
 * https://leetcode.cn/problems/sum-of-all-subset-xor-totals/description
 */
fn main() {}

pub struct Solution {}

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        Self::dfs(&nums, 0, 0, &mut ans);
        ans
    }

    pub fn dfs(nums: &[i32], i: usize, xor: i32, ans: &mut i32) {
        if i == nums.len() {
            *ans = *ans + xor;
        } else {
            // 要当前数
            Self::dfs(nums, i + 1, xor ^ nums[i], ans);
            // 不要当前数
            Self::dfs(nums, i + 1, xor, ans);
        }
    }

    // 首先长度为 n 的数组，子集的数量为 2^(n) 个
    // 通过观察得出
    // 如果某一位上全部为0，所有子集的异或 该位均为0
    // 如果某一位上存在1，子集的异或 该为 各有 2^(n-1) 即一半的0和1
    // 对结果产生影响的只有1
    // 将所有数字取或，就得到了所有存在1的位的结果，对该结果 左移 n - 1 即得到最终结果。
    pub fn subset_xor_sum_v1(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();

        for num in nums {
            res |= num;
        }

        res << (n - 1)
    }
}
