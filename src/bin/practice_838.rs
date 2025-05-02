fn main() {
    println!("{}", push_dominoes(".L.R...LR..L..".to_string()))
}

pub fn push_dominoes(dominoes: String) -> String {
    let mut s: Vec<char> = dominoes.chars().collect();
    let n = s.len();
    let mut i = 0;
    let mut left = 'L';
    while i < n {
        let mut j = i;
        while j < n && s[j] == '.' {
            j += 1;
        }
        let right = if j < n { s[j] } else { 'R' };
        if left == right {
            s[i..j].fill(right);
        } else if left == 'R' && right == 'L' {
            let mut k = j - 1;
            while i < k {
                s[i] = 'R';
                s[k] = 'L';
                i += 1;
                k -= 1;
            }
        }

        left = right;
        i = j + 1;
    }
    s.iter().collect()
}
