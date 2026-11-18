impl Solution {
    pub fn min_changes(s: String) -> i32 {
        // s is beautiful iff s matches the regex (00|11)*
        s.into_bytes()
            .as_chunks::<2>()
            .0
            .iter()
            .filter(|[a, b]| a != b)
            .count() as i32
    }
}

