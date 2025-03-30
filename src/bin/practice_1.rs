use leetcode_rust::generate_random_array;

fn main() {
    let max_size = 1000;
    let max_value = 1000;
    let test_times = 100000;

    let mut rng = rand::rng();

    for i in 0..test_times {
        let mut nums1 = generate_random_array(max_size, max_value, &mut rng);
        let mut nums2 = nums1.clone();

        println!("generate array {i}");

        nums1.sort();
        _insert_sort(&mut nums2);

        if nums1 != nums2 {
            println!("出错啦！");
            break;
        } else {
            println!("Pass Test {}", i)
        }
    }
}

// 选择排序
fn _select_sort(nums: &mut Vec<i32>) {
    for i in 0..nums.len() {
        let mut max = i;
        for j in i + 1..nums.len() {
            if nums[j] > nums[max] {
                max = j;
            }
        }
        nums.swap(i, max);
    }
}

// 冒泡排序
fn _bubble_sort(nums: &mut Vec<i32>) {
    for i in (2..=nums.len()).rev() {
        for j in 1..i {
            if nums[j - 1] > nums[j] {
                nums.swap(j - 1, j);
            }
        }
    }
}

// 插入排序
fn _insert_sort(nums: &mut Vec<i32>) {
    for i in 1..nums.len() {
        let mut j = i;
        let key = nums[j];
        while j > 0 && nums[j - 1] > key {
            nums[j] = nums[j - 1];
            j -= 1;
        }
        nums[j] = key;
    }
}
