impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let h: HashMap<_, Vec<_>> = strs.into_iter().fold(HashMap::new(), |mut h, s| {
            let key = s.bytes().fold([0; 26], |mut freq, b| {
                freq[(b - b'a') as usize] += 1;
                freq
            });
            h.entry(key).or_default().push(s);
            h
        });
        h.into_values().collect()
    }
}

