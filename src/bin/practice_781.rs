use std::collections::HashMap;

fn main() {
    println!("{}", num_rabbits(vec![0, 0]))
}

pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut map: HashMap<usize, usize> = HashMap::new();

    for ans in answers {
        let ans = ans as usize;
        if let Some(cnt) = map.get(&ans) {
            if *cnt == ans {
                map.remove(&ans);
            } else {
                map.insert(ans, cnt + 1);
            }
        } else if ans > 0 {
            res += ans + 1;
            map.insert(ans, 1);
        } else {
            res += 1;
        }
    }

    res as i32
}
