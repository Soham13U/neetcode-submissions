impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty(){
            return 0
        }
        let mut longest = 1;
        let mut set: HashSet<i32> = HashSet::from_iter(nums.clone());

        for n in nums{
                if !set.contains(&(n-1)){
                    let mut length = 1;
                    while set.contains(&(n+length)){
                        length += 1;
                    }
                    longest = longest.max(length);
                }
            } longest
        }
   
    }


/*
longest = 1
set
num 
2
20 
4 
10 
3 
5 

2 3 4 5 


*/