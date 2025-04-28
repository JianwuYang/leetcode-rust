fn main() {
    todo!();
}

pub struct Solution {}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n < 1 {
            return 0;
        }

        fn helper(limit: i32, col: i32, left: i32, right: i32) -> i32 {
            if col == limit {
                return 1;
            }

            let ban = col | left | right;

            let mut candidate = limit & (!ban);

            let mut place;

            let mut ans = 0;

            while candidate != 0 {
                place = candidate & (-candidate);

                candidate ^= place;

                ans += helper(
                    limit,
                    col | place,
                    (left | place) >> 1,
                    (right | place) << 1,
                );
            }
            ans
        }

        let limit = (1 << n) - 1;

        helper(limit, 0, 0, 0)
    }
}
