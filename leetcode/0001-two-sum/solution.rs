use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map: HashMap<i32, usize> = nums.iter().enumerate().map(|(i, &n)| (target - n, i)).collect();
        nums.iter().enumerate().filter_map(|(i, n)| match map.get(&n) {
            Some(&j) if i != j => Some(vec![i as i32, j as i32]),
            _ => None
        }).next().unwrap()
    }
}

