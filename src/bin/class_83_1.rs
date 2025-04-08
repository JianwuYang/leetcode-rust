/**
 * 1235. 规划兼职工作
 * https://leetcode.cn/problems/maximum-profit-in-job-scheduling/description/
 */

pub struct Solution {}

pub struct Job {
    start_time: i32,
    end_time: i32,
    profit: i32,
}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut jobs = Vec::with_capacity(n + 1);

        jobs.push(Job {
            start_time: 0,
            end_time: 0,
            profit: 0,
        });

        for i in 0..n {
            jobs.push(Job {
                start_time: start_time[i],
                end_time: end_time[i],
                profit: profit[i],
            });
        }

        jobs.sort_by(|a, b| a.start_time.cmp(&b.start_time));

        Self::f(&jobs, 1, 0)
    }

    // i当前任务，pre 上一次的任务
    pub fn f(jobs: &[Job], i: usize, pre: usize) -> i32 {
        if i == jobs.len() {
            return 0;
        }
        let mut ans = 0;

        // 当前工作不做，直接跳到下一个任务
        ans = ans.max(Self::f(jobs, i + 1, pre));

        // 当前任务的开始时间大于等于上一个任务的结束时间， 可以考虑做
        if jobs[i].start_time >= jobs[pre].end_time {
            ans = ans.max(Self::f(jobs, i + 1, i) + jobs[i].profit);
        }
        ans
    }

    pub fn job_scheduling_v1(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut jobs = Vec::with_capacity(n + 1);

        jobs.push(Job {
            start_time: 0,
            end_time: 0,
            profit: 0,
        });

        for i in 0..n {
            jobs.push(Job {
                start_time: start_time[i],
                end_time: end_time[i],
                profit: profit[i],
            });
        }

        jobs.sort_by(|a, b| a.start_time.cmp(&b.start_time));

        let mut dp = vec![vec![-1; n + 1]; n + 1];

        Self::f1(&jobs, 1, 0, &mut dp)
    }

    // i当前任务，pre 上一次的任务
    pub fn f1(jobs: &[Job], i: usize, pre: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if i == jobs.len() {
            return 0;
        }
        if dp[i][pre] != -1 {
            return dp[i][pre];
        }
        let mut ans = 0;

        // 当前工作不做，直接跳到下一个任务
        ans = ans.max(Self::f1(jobs, i + 1, pre, dp));

        // 当前任务的开始时间大于等于上一个任务的结束时间， 可以考虑做
        if jobs[i].start_time >= jobs[pre].end_time {
            ans = ans.max(Self::f1(jobs, i + 1, i, dp) + jobs[i].profit);
        }

        dp[i][pre] = ans;
        ans
    }

    // 严格依赖位置的版本
    pub fn job_scheduling_v2(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut jobs = Vec::with_capacity(n + 1);

        jobs.push(Job {
            start_time: 0,
            end_time: 0,
            profit: 0,
        });

        for i in 0..n {
            jobs.push(Job {
                start_time: start_time[i],
                end_time: end_time[i],
                profit: profit[i],
            });
        }

        jobs.sort_by(|a, b| a.start_time.cmp(&b.start_time));

        let mut dp = vec![0; n + 2];

        for i in (1..=n).rev() {
            for pre in (0..=n).rev() {
                if jobs[i].start_time >= jobs[pre].end_time {
                    dp[pre] = dp[pre].max(dp[i] + jobs[i].profit);
                }
            }
        }

        dp[0]
    }

    /**
     *
     */
    pub fn job_scheduling_v3(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();

        let mut jobs: Vec<Vec<i32>> = Vec::new();

        for i in 0..n {
            jobs.push(vec![start_time[i], end_time[i], profit[i]]);
        }

        jobs.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut dp = vec![0; n];

        dp[0] = jobs[0][2];

        for i in 1..n {
            let start = jobs[i][0];
            // 考虑当前工作
            dp[i] = jobs[i][2];
            if jobs[0][1] <= start {
                dp[i] += dp[Self::find(&jobs, i - 1, start)];
            }
            dp[i] = dp[i].max(dp[i - 1]);
        }

        dp[n - 1]
    }

    // 在0...1 范围上，找到结束时间 <= start 的最右下标
    pub fn find(jobs: &Vec<Vec<i32>>, i: usize, start: i32) -> usize {
        let mut ans = 0;
        let mut l = 0;
        let mut r = i;
        let mut m;
        while l <= r {
            m = l + ((r - l) >> 1);
            if jobs[m][1] <= start {
                ans = m;
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        ans
    }
}

fn main() {}
