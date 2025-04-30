impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(|x| x.ilog10()).filter(|n| n & 1 == 1).count() as i32
    }
}
