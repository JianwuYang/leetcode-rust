fn main() {
    let mut a = vec![3, 2, 1, 0];
    let mut b: Vec<i32> = vec![];
    let mut c: Vec<i32> = vec![];

    // let res = a.remove(0);
    // println!("{res}");

    // println!("{:?}", a);

    Solution::hanota(&mut a, &mut b, &mut c);
}

struct Solution {}

impl Solution {
    pub fn hanota(from: &mut Vec<i32>, other: &mut Vec<i32>, to: &mut Vec<i32>) {
        fn helper(num: usize, from: &mut Vec<i32>, other: &mut Vec<i32>, to: &mut Vec<i32>) {
            if num == 1 {
                to.push(from.pop().unwrap());
            }
            else {
                helper(num - 1, from, to, other);
                to.push(from.pop().unwrap());
                helper(num - 1, other, from, to);
            }
        }
        helper(from.len(), from, other, to);
    }
}
