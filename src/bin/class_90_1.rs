fn main() {
    
}

pub fn cutting_bamboo(bamboo_len: i32) -> i32 {
    if bamboo_len <= 3 {
        return bamboo_len - 1;
    }
    let a = bamboo_len / 3;
    let b = bamboo_len % 3;
    
    if b == 0 {
        // 如果余数是 0，直接返回 3^a
        return 3_i32.pow(a);
    }
    else if b == 1 {
        // 如果余数是 1，返回 3^(a-1) * 4
        return 3_i32.pow(a - 1) * 4;
    }
    else {
        // 如果余数是 2，返回 3^a * 2
        return 3_i32.pow(a) * 2;
    }
}