use std::cmp;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n: usize = cost.len();
        let mut dp = vec![0;n+1];
       
        
        for i in 2..=n{
            dp[i] = cmp::min(dp[i-1] + cost[i-1],dp[i-2]+cost[i-2]);
        }

        dp[n]
    }
}
/*
dp[0] = cost[0]
dp[1] = cmp::min(dp[0],cost[1])


dp[0] = 10
dp[1] = 10
dp[2] 



*/