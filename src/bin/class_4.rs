// 二分查找

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let ans = _binary_search(&nums, 4);
    println!("{ans}")
}

// 在有序数组中确认 num 是否存在
fn _binary_search(nums: &Vec<i32>, num: i32) -> bool {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l <= r {
        let m = l + ((r - l) >> 1);
        if nums[m] < num {
            l = m + 1;
        } else if nums[m] > num {
            r = m - 1;
        } else {
            return true;
        }
    }
    false
}

// 找出有序数组中 大于等于 num 的最左边位置
fn _find_gt_left(nums: &Vec<i32>, num: i32) -> Option<usize> {
    let mut ans = None;

    let mut l = 0;
    let mut r = nums.len() - 1;

    while l <= r {
        let m = l + ((r - l) >> 1);
        if nums[m] >= num {
            ans = Some(m);
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    ans
}

// 找出有序数组中 小于等于 num 的最右边位置
fn _find_lt_right(nums: &Vec<i32>, num: i32) -> Option<usize> {
    let mut ans = None;

    let mut l = 0;
    let mut r = nums.len() - 1;

    while l <= r {
        let m = l + ((r - l) >> 1);
        if nums[m] <= num {
            ans = Some(m);
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    ans
}

// 162. 寻找峰值
// https://leetcode.cn/problems/find-peak-element/description/
fn _find_peak_element(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    if n == 1 {
        return 0;
    }
    if nums[0] > nums[1] {
        return 0;
    }
    if nums[n - 1] > nums[n - 2] {
        return (n - 1) as i32;
    }

    let mut l = 1;
    let mut r = n - 2;

    while l <= r {
        let m = l + ((r - l) >> 1);
        if nums[m - 1] > nums[m] {
            r = m - 1;
        } else if nums[m] < nums[m + 1] {
            l = m + 1;
        } else {
            return m as i32;
        }
    }
    return -1;
}
