pub fn sort(stack: &mut Vec<i32>) {
    let mut deep = deep(stack);
    while deep > 0 {
        let max = max(stack, deep);
        let k = times(stack, deep, max);
        down(stack, deep, max, k);
        deep -= k;
    }
}

fn deep(stack: &mut Vec<i32>) -> usize {
    if stack.is_empty() {
        return 0;
    }
    let num = stack.pop().unwrap();
    let deep = deep(stack);
    stack.push(num);
    deep + 1
}

fn max(stack: &mut Vec<i32>, deep: usize) -> i32 {
    if deep == 0 {
        return i32::MIN;
    }
    let num = stack.pop().unwrap();
    let max = max(stack, deep - 1).max(num);
    stack.push(num);
    max
}

fn times(stack: &mut Vec<i32>, deep: usize, max: i32) -> usize {
    if deep == 0 {
        return 0;
    }
    let num = stack.pop().unwrap();
    let times = times(stack, deep - 1, max) + usize::from(num == max);
    stack.push(num);
    times
}

fn down(stack: &mut Vec<i32>, deep: usize, max: i32, k: usize) {
    if deep == 0 {
        // for _ in 0..k {
        // stack.push(max);
        // }
        stack.extend(std::iter::repeat(max).take(k));
    } else {
        let num = stack.pop().unwrap();
        down(stack, deep - 1, max, k);
        if num != max {
            stack.push(num);
        }
    }
}


#[cfg(test)]
mod tests {

    use rand::Rng;

    use super::*;

    #[test]
    fn test() {

        let mut rng = rand::rng();

        let mut stack = Vec::new();

        for _ in 0..10 {
            stack.push(rng.random_range(0..100));
        }

        sort(&mut stack);

        while !stack.is_empty() {
            println!("{}", stack.pop().unwrap());
        }

    }

}