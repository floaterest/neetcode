impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut h: HashMap<_, Vec<_>> = HashMap::new();
        for s in strs.into_iter() {
            let mut key = [0; 26];
            for b in s.bytes() {
                key[(b - b'a') as usize] += 1;
            }
            h.entry(key).or_default().push(s);
        }
        h.into_values().collect()
    }
}

