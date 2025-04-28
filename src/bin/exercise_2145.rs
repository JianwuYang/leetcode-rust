fn main() {
    todo!();
}

pub struct Solution {}

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (mut max_num, mut min_num) = (0, 0);
        let mut sum = 0;
        for diff in differences {
            sum += diff;
            max_num = max_num.max(sum);
            min_num = min_num.min(sum);
            if max_num - min_num > upper - lower {
                return 0;
            }
        }

        (upper - lower) - (max_num - min_num) + 1
    }
}
