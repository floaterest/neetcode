impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        num + num.checked_ilog2().unwrap_or_default() as i32
            - (1..32).map(|i| num >> i).sum::<i32>()
    }
}


