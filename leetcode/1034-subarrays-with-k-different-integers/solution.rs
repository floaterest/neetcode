use std::collections::HashMap;
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        // O(n) sliding window
        let k = k as usize;
        let (mut i, mut m, mut h) = (0, 0, HashMap::new());
        // h counts frequency of A[m..=j]
        // if A[m..=j] has exactly k, then
        //   i is minimum index such that A[i..=j] is valid
        //   m is maximum index such that A[m..=j] is valid
        //   therefore, there are exactly m - i + 1 valid subarrays that ends at j
        nums.iter()
            .enumerate()
            .map(|(_j, a)| {
                *h.entry(*a).or_insert(0) += 1;
                if h.len() > k {
                    assert_eq!(h.len(), k + 1);
                    assert_eq!(h.remove(&nums[m]).unwrap(), 1);
                    m += 1;
                    i = m;
                }
                if h.len() == k {
                    while h[&nums[m]] > 1 {
                        h.entry(nums[m]).and_modify(|e| *e -= 1);
                        m += 1;
                    }
                    m - i + 1
                } else {
                    0
                }
            })
            .sum::<usize>() as i32
    }
}


