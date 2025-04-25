use std::{collections::HashMap, iter::once};
impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        nums.into_iter()
            .scan(0, |acc, x| {
                *acc += if x % m == k { 1 } else { 0 };
                Some(*acc % m)
            })
            // count number of subarrays with sum == k mod m
            .scan(once((0, 1)).collect::<HashMap<_, _>>(), |h, sj| {
                let si = (sj - k + m) % m;
                let c = *h.get(&si).unwrap_or(&0);
                *h.entry(sj).or_insert(0) += 1;
                Some(c)
            })
            .sum()
    }
}


