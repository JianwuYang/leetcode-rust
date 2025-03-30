use std::collections::VecDeque;

// 2360. 图中的最长环
// https://leetcode.cn/problems/longest-cycle-in-a-graph/description/
fn main() {
    let edges = vec![3, 3, 4, 2, 3];
    let ans = longest_cycle(edges);
    println!("{}", ans)
}

fn longest_cycle(edges: Vec<i32>) -> i32 {
    let n = edges.len();

    let mut indegree = vec![0; n];

    for i in 0..n {
        if edges[i] != -1 {
            indegree[edges[i] as usize] += 1;
        }
    }

    let mut visited = vec![false; n];

    let mut queue = VecDeque::<usize>::new();

    for i in 0..indegree.len() {
        if indegree[i] == 0 {
            queue.push_back(i);
        }
    }

    while let Some(cur) = queue.pop_front() {
        if !visited[cur] {
            visited[cur] = true;

            let to = edges[cur];

            if to != -1 {
                let to = to as usize;
                indegree[to] -= 1;

                if indegree[to] == 0 {
                    queue.push_back(to);
                }
            }
        }
    }

    let mut ans = -1;

    for i in 0..visited.len() {
        if !visited[i] {
            visited[i] = true;

            let mut size = 1;
            let mut cur = edges[i];

            while cur != -1 && !visited[cur as usize] {
                visited[cur as usize] = true;
                size += 1;
                cur = edges[cur as usize];
            }

            ans = std::cmp::max(ans, size);
        }
    }

    ans
}
