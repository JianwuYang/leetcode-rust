fn main() {
    Solution::find_rotate_steps("pqwcx".to_string(), "cpqwx".to_string());
}

pub struct Solution {}

impl Solution {
    pub fn find_rotate_steps(r: String, k: String) -> i32 {
        let n = r.len();
        let m = k.len();

        let maxc = 26;

        let mut size = vec![0; maxc];
        let mut position = vec![vec![0; n]; maxc];

        let mut ring = vec![0; n];
        let mut key = vec![0; m];

        for (i, c) in r.chars().enumerate() {
            let v = c as usize - 'a' as usize;
            position[v][size[v]] = i;
            size[v] += 1;
            ring[i] = v;
        }

        for (i, c) in k.chars().enumerate() {
            key[i] = c as usize - 'a' as usize;
        }

        // dp[i][j] 指针在 ring 的 j 位置 解决 i.. 之后的 key 所需要的步数
        // dp[i][j] 假设 i 处字符为 x，则 ring 旋转到 x 处的步数 + dp[i + 1][x(位置)] 取最小的

        // 基本情况 dp[m][0..n] = 0，指针已经来到了ring的最后位置已经完成了

        let mut dp = vec![vec![0; n]; m + 1];

        for i in (0..m).rev() {
            for j in 0..n {
                // 如果 key[i] == ring[j] 就不用转动了，直接处理了
                dp[i][j] = i32::MAX;
                if key[i] == ring[j] {
                    dp[i][j] = dp[i + 1][j] + 1;
                } else {
                    let p1 = Self::clock(&size, &position, i, key[j]);
                    let distance1 = if p1 > i { p1 - i } else { p1 + n - i };
                    let p2 = Self::counterclock(&size, &position, i, key[j]);
                    let distance2 = if i > p2 { i - p2 } else { i + n - p2 };
                    dp[i][j] = (distance1 as i32 + dp[p1][j]).min(distance2 as i32 + dp[p2][j]);
                }
            }
        }
        dp[0][0]
    }

    // 顺时针寻找 最近的 v 在什么位置
    pub fn clock(size: &[usize], position: &Vec<Vec<usize>>, i: usize, v: usize) -> usize {
        let mut l = 0;
        let mut r = size[v] - 1;
        let sorted = &position[v];
        let mut find = -1;
        // 找到 > i 尽量靠左的下标
        while l <= r {
            let m = l + ((r - l) >> 1);
            if sorted[m] > i {
                find = m as i32;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        if find != -1 {
            return sorted[find as usize];
        } else {
            return sorted[0];
        }
    }

    // 顺时针寻找 最近的 v 在什么位置
    pub fn counterclock(size: &[usize], position: &Vec<Vec<usize>>, i: usize, v: usize) -> usize {
        let mut l = 0;
        let mut r = size[v] - 1;
        let sorted = &position[v];
        let mut find = -1;
        // 找到 < i 尽量靠右的下标
        while l <= r {
            let m = l + ((r - l) >> 1);
            if sorted[m] < i {
                find = m as i32;
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        if find != -1 {
            return sorted[find as usize];
        } else {
            return sorted[size[v] - 1];
        }
    }
}
