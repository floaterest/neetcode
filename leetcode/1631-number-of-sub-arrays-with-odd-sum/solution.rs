impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: u32 = 1_000_000_007;
        let n = arr.len() as u32;
        // number of odd prefix sums
        let count: u32 = arr
            .into_iter()
            .scan(0, |parity, x| {
                // parity ∈ {0,1} of the current prefix sum
                *parity ^= x as u32 & 1;
                Some(*parity)
            })
            .sum();
        // n - count + 1 is number of even prefix sums (including the empty array)
        //
        // Every odd-sum subarray can be seen as the intersection of an even-sum prefix subarray and
        // an odd-sum prefix subarray. In particular, we have the following one-to-one
        // correspondence:
        //   { even-sum prefix subarray } × { odd-sum prefix subarray } ≅ { odd-sum subarray }
        (((n - count + 1) * count) % MOD) as i32
    }
}

