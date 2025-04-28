fn main() {
    for apple in 0..100 {
        let ans1 = bags1(apple);
        let ans2 = bags2(apple);
        if ans1 != ans2 {
            println!("Error,apple: {}, ans1: {}, ans2: {}", apple, ans1, ans2);
            break;
        }
    }
}

pub fn bags1(apple: i32) -> i32 {
    f(apple).unwrap_or(-1)
}

pub fn f(rest: i32) -> Option<i32> {
    if rest < 0 {
        return None;
    }
    if rest == 0 {
        return Some(0);
    }

    let p1 = f(rest - 8);
    let p2 = f(rest - 6);

    match (p1, p2) {
        (Some(a), Some(b)) => Some(a.min(b) + 1),
        (Some(a), None) => Some(a + 1),
        (None, Some(b)) => Some(b + 1),
        (None, None) => None,
    }
}

pub fn bags2(apple: i32) -> i32 {
    if (apple & 1) != 0 {
        return -1;
    }

    if apple < 18 {
        return match apple {
            0 => 0,
            6 | 8 => 1,
            12 | 14 | 16 => 2,
            _ => -1,
        };
    }

    (apple - 18) / 8 + 3
}
