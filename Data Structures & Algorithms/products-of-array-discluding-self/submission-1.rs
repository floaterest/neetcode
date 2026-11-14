impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut pre = 1;
        for (i, &n) in nums.iter().enumerate() {
            res[i] = pre;
            pre *= n;
        }
        let mut pos = 1;
        for (i, &n) in nums.iter().enumerate().rev() {
            res[i] *= pos;
            pos *= n;
        }
        res
    }
}

