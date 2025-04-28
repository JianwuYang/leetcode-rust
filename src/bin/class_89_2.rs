fn main() {
    todo!();
}

pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    let mut diff = Vec::with_capacity(costs.len());
    for cost in &costs {
        sum += cost[0];
        diff.push(cost[1] - cost[0]);
    }
    diff.sort();
    sum + diff.iter().take(diff.len() / 2).sum::<i32>()
}
