use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // println!("{s}");
        s.chars()
            .enumerate()
            .scan(HashMap::new(), |map, (i, c)| {
                if let Some(j) = map.get(&c) {
                    *map = map
                        .iter()
                        .filter_map(|(&k, &v)| if v > *j { Some((k, v)) } else { None })
                        .collect();
                }
                map.insert(c, i);
                Some(map.clone())
            })
            // .inspect(|map| println!("{map:?}"))
            .map(|map| map.len())
            .max()
            .unwrap_or(0) as i32
    }
}

