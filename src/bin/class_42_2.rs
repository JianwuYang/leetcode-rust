fn main() {
    for i in 0..=50 {
        println!("{}:{}", i, win2(i));
    }
}
/* 草一共有n的重量，两只牛轮流吃草，A牛先吃，B牛后吃
 * 每只牛在自己的回合，吃草的重量必须是4的幂，1、4、16、64....
 * 谁在自己的回合正好把草吃完谁赢，根据输入的n，返回谁赢
 */
pub fn win1(n: i32) -> String {
    f(n, "A")
}

pub fn f(rest: i32, cur: &str) -> String {
    let enemy = match cur {
        "A" => "B",
        _ => "A",
    };

    if rest < 5 {
        return match rest {
            0 | 2 => enemy.to_string(),
            _ => cur.to_string(),
        };
    }

    let mut pick = 1;
    while pick <= rest {
        if f(rest - pick, enemy) == cur {
            return cur.to_string();
        }
        pick *= 4;
    }

    enemy.to_string()
}

pub fn win2(n: i32) -> String {
    if n % 5 == 0 || n % 5 == 2 {
        "B".to_string()
    } else {
        "A".to_string()
    }
}
