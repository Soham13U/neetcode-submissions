impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i,&n) in nums.iter().enumerate(){
            map.insert(n,i);
        }

        for (i,&n) in nums.iter().enumerate(){
            let diff = target - n;
            if let Some(&j) = map.get(&diff){
                if i!=j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}
/*
store in map val -> ind

*/