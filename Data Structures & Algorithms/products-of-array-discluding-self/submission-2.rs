impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        // prefix product
        let mut a = 1;
        for &n in &nums {
            res.push(a);
            a *= n;
        }
        // postfix product
        a = 1;
        for (i, &n) in nums.iter().enumerate().rev() {
            res[i] *= a;
            a *= n;
        }
        res
    }
}

