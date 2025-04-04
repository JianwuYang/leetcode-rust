use std::io::{self, BufRead};

/**
 * 计算数组的小和
 * https://www.nowcoder.com/practice/edfe05a1d45c4ea89101d936cac32469
 */
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buf = String::new();

    while handle.read_line(&mut buf)? > 0 {
        let n: usize = buf.trim().parse().unwrap();
        buf.clear();

        handle.read_line(&mut buf)?;

        let mut arr: Vec<i32> = buf
            .split_whitespace()
            .take(n)
            .map(|s| s.parse().unwrap())
            .collect();

        buf.clear();

        let mut helper = vec![0; n];

        let sum = small_sum(&mut arr, &mut helper, 0, n - 1);

        println!("{}", sum);
    }

    Ok(())
}

fn small_sum(arr: &mut [i32], helper: &mut [i32], l: usize, r: usize) -> i64 {
    if l == r {
        return 0;
    }

    let m = l + ((r - l) >> 1);
    let mut sum = small_sum(arr, helper, l, m);
    sum += small_sum(arr, helper, m + 1, r);
    sum += merge(arr, helper, l, m, r);
    sum
}

fn merge(arr: &mut [i32], helper: &mut [i32], l: usize, m: usize, r: usize) -> i64 {
    let mut cur1 = l;
    let mut cur2 = m + 1;

    let mut sum = 0i64;
    let mut i = l;
    while cur1 <= m && cur2 <= r {
        if arr[cur1] <= arr[cur2] {
            helper[i] = arr[cur1];
            sum += arr[cur1] as i64 * (r - cur2 + 1) as i64;
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

    sum
}
