impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // assume items are numbered 1, 2, ...
        // dp[i] is best rob for items 1..i
        // dp[i] is either
        //   dp[i - 2] + v_i if take the ith item
        //   dp[i - 1]       if skip the ith item
        nums.iter().fold((0, 0), |(a1, a2), v| (a1.max(a2 + v), a1)).0
    }
}


