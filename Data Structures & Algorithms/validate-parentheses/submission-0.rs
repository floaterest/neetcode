impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st = Vec::with_capacity(s.len());
        let h = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
        for ch in s.chars() {
            if let Some(c) = h.get(&ch) {
                st.push(c);
                continue;
            } else if st.pop_if(|&mut &c| c == ch).is_none() {
                return false;
            }
        }
        st.is_empty()
    }
}
