use rand::{Rng, rngs::ThreadRng};

pub fn generate_random_array(max_size: usize, max_value: i32, rng: &mut ThreadRng) -> Vec<i32> {
    let len = rng.random_range(0..max_size);
    let mut res: Vec<i32> = vec![0; len];
    for i in 0..len {
        res[i] = rng.random_range(0..max_value);
    }
    res
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {

    #[inline] // 尽可能内联的来优化性能
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}