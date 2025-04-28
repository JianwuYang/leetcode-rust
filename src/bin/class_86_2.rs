use std::collections::HashMap;

fn main() {
    let req_skills: Vec<String> = ["java", "nodejs", "reactjs"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let people: Vec<Vec<String>> = [vec!["java"], vec!["nodejs"], vec!["nodejs", "reactjs"]]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();

    let ans = Solution::smallest_sufficient_team(req_skills, people);
    println!("{:?}", ans);
}

pub struct Solution {}

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let skill_to_id: HashMap<_, _> = req_skills
            .iter()
            .enumerate()
            .map(|(i, skill)| (skill, i))
            .collect();

        let skill_count = req_skills.len();

        let people_skills: Vec<_> = people
            .iter()
            .map(|p| {
                p.iter()
                    .fold(0, |status, skill| status | 1 << skill_to_id[skill])
            })
            .collect();

        let mut dp = vec![None; 1 << skill_count];
        dp[0] = Some(Vec::new());

        for i in 0..(1 << skill_count) {
            if dp[i].is_none() {
                continue;
            }

            for (j, &person_skill) in people_skills.iter().enumerate() {
                let new_skill_mask = i | person_skill;
                if dp[new_skill_mask].is_none()
                    || dp[new_skill_mask].as_ref().unwrap().len()
                        > dp[i].as_ref().unwrap().len() + 1
                {
                    let mut new_team = dp[i].as_ref().unwrap().clone();
                    new_team.push(j as i32);
                    dp[new_skill_mask] = Some(new_team);
                }
            }
        }

        dp[(1 << skill_count) - 1].clone().unwrap()
    }
}
