use std::cmp;
impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = heights.len() - 1;
        let mut maxArea = 0;
        
        while l < r{
            let area = ((r-l) as i32)*cmp::min(heights[l],heights[r]) ;
            maxArea = cmp::max(maxArea,area);
            if heights[l] < heights[r]{
                l += 1;
            }
            else{
                r -= 1;
            }
        }


        maxArea 
    }
}
/*
l = 0
r = len - 1
while l < r

    calc area -> (r-l)*min(a[l],a[r])
    update with max
    if a[l] < a[r]
        l++

    else
        r--



*/
