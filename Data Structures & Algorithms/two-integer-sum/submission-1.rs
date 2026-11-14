impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::with_capacity(nums.len());
        nums.into_iter()
            .enumerate()
            .find_map(|(i, n)| match h.get(&(target - n)) {
                Some(&j) => Some(vec![j as i32, i as i32]),
                _ => {
                    h.entry(n).or_insert(i);
                    None
                }
            })
            .unwrap()
    }
}

