use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
// setAll功能的哈希表
// 测试链接 : https://www.nowcoder.com/practice/7c4559f138e74ceb9ba57d76fd169967
fn main() -> io::Result<()> {

    let mut br = BufReader::new(io::stdin());
    let mut bo = BufWriter::new(io::stdout());

    
    let mut map = SetAllMap::new();

    let mut buf = String::new();

    while br.read_line(&mut buf)? > 0 {

        let n = buf.trim().parse::<usize>().unwrap();

        for _ in 0..n {
            buf.clear();

            br.read_line(&mut buf)?;

            let parts: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

            let op: i32 = parts[0];

            if op == 1 {
                let k: i32 = parts[1];
                let v: i32 = parts[2];
                map.set(k, v);
            }
            else if op == 2 {
                let k: i32 = parts[1];
                bo.write_all(format!("{}\n", map.get(&k)).as_bytes()).unwrap();
            }
            else if op == 3 {
                let v: i32 = parts[1];
                map.set_all(v);
            }
        }
    }
    bo.flush().unwrap();

    Ok(())
}

struct SetAllMap {
    map: HashMap<i32, (i32, i32)>,
    cnt: i32,
    cap: Option<i32>,
    cap_value: i32,
}

impl SetAllMap {
    fn new() -> Self {
        SetAllMap {
            map: HashMap::new(),
            cnt: 0,
            cap: None,
            cap_value: -1,
        }
    }

    fn set(self: &mut Self, k: i32, v: i32) {
        self.cnt += 1;
        self.map.insert(k, (self.cnt, v));
    }

    fn get(self: &Self, k: &i32) -> i32 {
        if let Some(v) = self.map.get(k) {
            if let Some(cap) = self.cap {
                if cap >= v.0 {
                    return self.cap_value;
                } else {
                    return v.1;
                }
            } else {
                return v.1;
            }
        } else {
            return -1;
        }
    }

    fn set_all(self: &mut Self, v: i32) {
        self.cap = Some(self.cnt);
        self.cap_value = v;
    }
}
