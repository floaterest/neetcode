impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |s, &n| {
                *s = *s + n;
                Some(*s)
            })
            .collect()
    }
}


