use crate::common::Solution;

impl Solution {
    pub fn hanota(from: &mut Vec<i32>, other: &mut Vec<i32>, to: &mut Vec<i32>) {

        dbg!("from: {:?}, to: {:?}, other: {:?}", &from, &to, &other);


        let n = from.len();

        // 先把 from 当中 n - 1 移动到 other
        // 然后把 from 当中剩下的 1 移动到 to
        // 再把 other 当中的 n - 1 移动到 to

        // 如果 from 当中只有1个的话，直接移动到 to

        if n == 1 {
            to.push(from.pop().unwrap());
        } else {
            let res = from.remove(0);
            Self::hanota(from, to, other);
            to.push(res);
            Self::hanota(other, from, to);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let mut a = vec![3, 2, 1, 0];
        let mut b: Vec<i32> = vec![];
        let mut c: Vec<i32> = vec![];

        // let res = a.remove(0);
        // println!("{res}");


        // println!("{:?}", a);


        Solution::hanota(&mut a, &mut b, &mut c);
    }
}
