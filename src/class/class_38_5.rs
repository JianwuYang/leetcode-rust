pub fn reverse(stack: &mut Vec<i32>) {
    if stack.is_empty() {
        return;
    }
    let num = bottom_out(stack);
    reverse(stack);
    stack.push(num);
}

pub fn bottom_out(stack: &mut Vec<i32>) -> i32 {
    let ans = stack.pop().unwrap();
    if stack.is_empty() {
        return ans;
    } else {
        let last = bottom_out(stack);
        stack.push(ans);
        return last;
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {

        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        reverse(&mut stack);

        while !stack.is_empty() {
            let cur = stack.pop().unwrap();
            println!("{cur}");
        }

    }
}