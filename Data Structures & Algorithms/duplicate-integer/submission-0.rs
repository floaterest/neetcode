use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let a: HashSet<_> = nums.iter().collect();
        a.len() != nums.len()
    }
}
