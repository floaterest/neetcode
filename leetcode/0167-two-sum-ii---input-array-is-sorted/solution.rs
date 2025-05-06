impl Solution {
    pub fn two_sum(ns: Vec<i32>, t: i32) -> Vec<i32> {
        for (i, &a) in ns.iter().enumerate() {
            if let Ok(j) = ns[i + 1..].binary_search(&(t - a)) {
                return [i + 1, i + j + 2].into_iter().map(|x| x as i32).collect();
            }
        }
        unreachable!()
    }
}


