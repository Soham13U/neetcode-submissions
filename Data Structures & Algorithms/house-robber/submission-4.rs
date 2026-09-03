use std::cmp;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {

        if nums.len() < 2{
            return nums[0]
        }
        let mut dp = vec![0;nums.len()];
        dp[0] = nums[0 as usize];
        dp[1] = cmp::max(nums[0 as usize],nums[1 as usize]);      

        for i in 2..nums.len(){
            dp[i] = cmp::max(nums[i] + dp[i-2], dp[i-1]);
        }

        return dp[nums.len()-1]

    }
  
}
/*


*/