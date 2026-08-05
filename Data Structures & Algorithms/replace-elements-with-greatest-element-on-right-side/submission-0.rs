use std::cmp;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut curr_max: i32 = arr[arr.len()-1];
        let mut a: Vec<i32> = arr.clone();

        for i in (0..a.len()).rev(){
            if i == a.len()-1 {
                a[i] = -1;
            }
            else {
                let temp = a[i];
                a[i] = curr_max;
                curr_max = cmp::max(curr_max,temp);
            }
        }

        a
    }
}
