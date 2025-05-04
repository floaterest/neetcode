use std::collections::HashMap;
impl Solution {
    pub fn num_equiv_domino_pairs(ds: Vec<Vec<i32>>) -> i32 {
        let h = ds.into_iter().fold(HashMap::new(), |mut h, v|{
            let (a,b) = (v[0], v[1]);
            *h.entry(if a <= b {(a,b)} else {(b,a)}).or_insert(0) += 1;
            h
        });
        h.values().map(|i| i * (i-1) / 2).sum::<i32>()
    }
}


