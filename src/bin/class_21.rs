/**
 * 归并排序
 */
fn main() {
    let arr = Solution::sort_array(vec![5, 2, 3, 1]);

    println!("final arr: {:?}", arr)
}

pub struct Solution {}

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();

        let mut arr = nums;

        let mut helper = vec![0; n];

        Solution::sort(&mut arr, 0, n - 1, &mut helper);

        arr
    }

    pub fn sort(arr: &mut Vec<i32>, l: usize, r: usize, helper: &mut Vec<i32>) {
        if l == r {
            return;
        }

        let m = l + ((r - l) >> 1);
        Solution::sort(arr, l, m, helper);
        Solution::sort(arr, m + 1, r, helper);
        Solution::merge(arr, l, m, r, helper);
    }

    pub fn merge(arr: &mut Vec<i32>, l: usize, m: usize, r: usize, helper: &mut Vec<i32>) {
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
    }

    pub fn sort_array_v1(mut arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut helper = vec![0; n];
        let mut step = 1;
        let (mut l, mut m, mut r);
        while step < n {
            l = 0;
            while l < n {
                m = l + step - 1;
                if m + 1 >= n {
                    break;
                }
                r = std::cmp::min(l + (step << 1) - 1, n - 1);
                Self::merge(&mut arr, l, m, r, &mut helper);
                l = r + 1;
            }
            step <<= 1;
        }
        arr
    }
}
