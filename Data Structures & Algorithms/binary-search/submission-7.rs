impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len()-1;
        while l<=r {
            let mut mid = l + (r - l) / 2;

           if nums[mid] == target {
                return mid as i32; 
            } else if nums[mid] > target {
                
                if mid == 0 {
                    break;
                }
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        -1
    }
}
