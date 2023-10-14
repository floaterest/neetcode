impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        use std::collections::{HashMap, HashSet};
        let vs: HashSet<_> = "aeiou".chars().collect();
        let bs: HashMap<_, _> = vs.iter().cloned().enumerate().map(|(i, c)| (c, 1 << i)).collect();
        let states: Vec<_> = s
            .chars()
            .scan(0, |state, c| {
                if vs.contains(&c) {
                    *state = *state ^ bs.get(&c).unwrap();
                }
                Some(*state)
            })
            .collect();
        // even count => start and end char have same state
        (0..1 << bs.len())
            .flat_map(|i| {
                let eq = |&s| s == i;
                let cast = |i| i as i32;
                let a = if i == 0 { Some(-1) } else { states.iter().position(eq).map(cast) };
                let b = states.iter().rposition(eq).map(cast);
                a.and_then(|a| b.and_then(|b| Some(b - a)))
            })
            .max()
            .unwrap_or(0)
    }
}

