impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut i = 0;
        for x in arr.into_iter() {
            i = if x & 1 == 1 { i + 1 } else { 0 };
            if i == 3 { return true; }
        }
        false
    }
}
