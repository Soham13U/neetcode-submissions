impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        let s = nums.len();
        for n in nums{
            set.insert(n);
        }
        return set.len() != s;
    }
}
