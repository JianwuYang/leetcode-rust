use std::{
    f64, io::{self, BufRead}, time::{SystemTime, UNIX_EPOCH}
};

/**
 * https://www.luogu.com.cn/problem/P1177
 * 排序模板
 *
 * 快速排序
 */
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buf = String::new();

    while handle.read_line(&mut buf)? > 0 {
        let n = buf.trim().parse::<usize>().unwrap();

        buf.clear();

        handle.read_line(&mut buf)?;

        let mut arr: Vec<i32> = buf
            .split_whitespace()
            .take(n)
            .map(|s| s.parse().unwrap())
            .collect();

        buf.clear();

        quick_sort_v1(&mut arr, 0, n - 1);

        for num in arr {
            print!("{num} ");
        }
        println!();
    }

    Ok(())
}

fn quick_sort_v1(arr: &mut [i32], l: usize, r: usize) {
    if l >= r {
        return;
    }
    let x = arr[generate_random(l, r)];
    let (left, right) = partition_v1(arr, l, r, x);

    if left > 0 {
        quick_sort_v1(arr, l, left - 1);
    }
    quick_sort_v1(arr, right + 1, r);
}

fn partition_v1(arr: &mut [i32], mut l: usize, mut r: usize, x: i32) -> (usize, usize) {
    let mut i = l;
    while i <= r {
        if arr[i] == x {
            i += 1;
        } else if arr[i] < x {
            arr.swap(i, l);
            l += 1;
            i += 1;
        } else {
            arr.swap(i, r);
            r -= 1;
        }
    }
    (l, r)
}

fn generate_random(l: usize, r: usize) -> usize {
    let now = SystemTime::now();

    let duration = now.duration_since(UNIX_EPOCH).unwrap();

    let millis = duration.as_millis();

    let random = (millis % 100) as f64 / 100  as f64;

    l + (r - l) * random as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut nums = vec![4, 2, 4, 5, 1];
        let n = nums.len();
        quick_sort_v1(&mut nums, 0, n - 1);
        println!("{:?}", nums);
    }
}
