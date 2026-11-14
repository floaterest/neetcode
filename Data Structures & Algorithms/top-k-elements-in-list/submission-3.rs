impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // frequency map
        let mut freq: HashMap<_, usize> = HashMap::new();
        let len = nums.len();
        for n in nums.into_iter() {
            *freq.entry(n).or_default() += 1;
        }
        // distribute to bins
        let mut bins = vec![vec![]; len];
        for (n, c) in freq.into_iter() {
            assert!((0..len).contains(&(len - c)));
            bins[len - c].push(n);
        }
        bins.into_iter().flatten().take(k as usize).collect()
    }
}
