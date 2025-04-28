use rand::distr::{
    Alphanumeric, Distribution,
    uniform::{UniformSampler, UniformUsize},
};

fn main() {
    let len_dist = UniformUsize::new(1, 8).unwrap();
    let char_len_dist = UniformUsize::new(1, 5).unwrap();

    for i in 0..2000 {
        let strs = generate_random_string_arr(&len_dist, &char_len_dist);

        let ans1 = way1(strs.clone());
        let ans2 = way2(strs);

        if ans1 != ans2 {
            println!("出错了！ans1: {ans1}, ans2: {ans2}");
            break;
        }

        if i % 100 == 0 {
            println!("处理第{i}条数据");
        }
    }
}

fn generate_random_string_arr(
    len_dist: &UniformUsize,
    char_len_dist: &UniformUsize,
) -> Vec<String> {
    let mut rng = rand::rng();

    let len = len_dist.sample(&mut rng);
    let mut ans = Vec::with_capacity(len);

    for _ in 0..len {
        let str_len = char_len_dist.sample(&mut rng);
        let str = Alphanumeric
            .sample_iter(&mut rng)
            .take(str_len)
            .map(char::from)
            .collect();
        ans.push(str);
    }

    ans
}

pub fn way1(mut strs: Vec<String>) -> String {
    let mut ans = vec![];
    f(&mut strs, 0, &mut ans);
    ans.sort();
    ans[0].clone()
}

fn f(strs: &mut Vec<String>, i: usize, ans: &mut Vec<String>) {
    if i == strs.len() {
        let mut tmp = String::new();
        for str in strs {
            tmp.push_str(str);
        }
        ans.push(tmp);
    } else {
        for j in i..strs.len() {
            strs.swap(i, j);
            f(strs, i + 1, ans);
            strs.swap(i, j);
        }
    }
}

pub fn way2(mut strs: Vec<String>) -> String {
    strs.sort_by(|a, b| format!("{a}{b}").cmp(&format!("{b}{a}")));
    strs.join("")
}

pub fn largest_number(nums: Vec<i32>) -> String {
    let mut strs: Vec<String> = nums.iter().map(i32::to_string).collect();
    strs.sort_by(|a, b| format!("{b}{a}").cmp(&format!("{a}{b}")));
    if strs[0] == "0" {
        return "0".to_string();
    }
    strs.join("")
}
