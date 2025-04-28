fn main() {
    let a = 64;
    let b = 24;
    println!("{}", lcm(a, b));
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
