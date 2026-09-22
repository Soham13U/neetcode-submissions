impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
            let l = nums.len();
            let mut pre_v = vec![1; l];
            let mut pos_v = vec![1; l];
            let mut output = Vec::new();
           
            pre_v.push(1);
            for i in 1..l{
                pre_v[i] = pre_v[i-1] * nums[i-1];
            }
            for i in (0..l-1).rev(){
                pos_v[i] = pos_v[i+1] * nums[i+1];
               
            }

            for i in 0..l{
                output.push(pos_v[i] * pre_v[i]);
            }

            output
    }

}