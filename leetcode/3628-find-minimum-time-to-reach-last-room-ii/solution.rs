use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
impl Solution {
    fn neighbors(
        (x, y): (usize, usize),
        (w, h): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        [
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
            (x < w - 1).then_some((x + 1, y)),
            (y < h - 1).then_some((x, y + 1)),
        ]
        .into_iter()
        .flatten()
    }
    pub fn min_time_to_reach(ts: Vec<Vec<i32>>) -> i32 {
        let mut h: BinaryHeap<_> = [(Reverse(0), (0, 0), 1)].into();
        let mut w: HashMap<_, _> = [((0, 0), 0)].into();
        let (n, m) = (ts.len(), ts[0].len());
        // modified dijstra
        while let Some((Reverse(wu), u, d)) = h.pop() {
            if u == (n - 1, m - 1) {
                return wu; // reached destination
            }
            if wu > w.get(&u).copied().unwrap_or(i32::MAX) {
                continue; // current path is not optimal
            }
            for (wv, v) in Self::neighbors(u, (n, m)).map(|(x, y)| (ts[x][y].max(wu) + d, (x, y))) {
                if wv < w.get(&v).copied().unwrap_or(i32::MAX) {
                    h.push((Reverse(wv), v, 3 - d));
                    *w.entry(v).or_default() = wv;
                }
            }
        }
        unreachable!()
    }
}


