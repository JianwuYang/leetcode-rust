/**
 * 493. 翻转对
 * https://leetcode.cn/problems/reverse-pairs/description/
 */
fn main() {}

pub struct Solution {}

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut helper = vec![0; n];

        Self::f(&mut nums, &mut helper, 0, n - 1)
    }

    pub fn f(arr: &mut [i32], helper: &mut [i32], l: usize, r: usize) -> i32 {
        if l == r {
            return 0;
        }
        let m = l + ((r - l) >> 1);

        Self::f(arr, helper, l, m)
            + Self::f(arr, helper, m + 1, r)
            + Self::merge(arr, helper, l, m, r)
    }

    pub fn merge(arr: &mut [i32], helper: &mut [i32], l: usize, m: usize, r: usize) -> i32 {
        let mut ans = 0;

        let mut j = m + 1;
        for i in l..=m {
            while j <= r && arr[i] as i64 > arr[j] as i64 * 2 {
                j += 1;
            }
            ans += j - m - 1;
        }

        let mut cur1 = l;
        let mut cur2 = m + 1;

        let mut i = l;

        while cur1 <= m && cur2 <= r {
            if arr[cur1] <= arr[cur2] {
                helper[i] = arr[cur1];
                i += 1;
                cur1 += 1;
            } else {
                helper[i] = arr[cur2];
                i += 1;
                cur2 += 1;
            }
        }

        while cur1 <= m {
            helper[i] = arr[cur1];
            i += 1;
            cur1 += 1;
        }

        while cur2 <= r {
            helper[i] = arr[cur2];
            i += 1;
            cur2 += 1;
        }

        for i in l..=r {
            arr[i] = helper[i];
        }

        ans as i32
    }
}
