use std::{collections::HashSet, iter::successors};
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // O(n) time
        let set: HashSet<_> = nums.into_iter().collect();
        set.iter()
            .filter(|&a| !set.contains(&(a - 1)))
            .map(|&a| successors(Some(a), |&a| Some(a + 1)).take_while(|a| set.contains(a)).count())
            .max().unwrap_or(0) as i32
    }
}


