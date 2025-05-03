impl Solution {
    pub fn min_domino_rotations(ts: Vec<i32>, bs: Vec<i32>) -> i32 {
        let (t0, b0) = (ts[0], bs[0]);
        // AB is A0 at row B
        let (mut tt, mut tb, mut bt, mut bb) = (Some(0), Some(0), Some(0), Some(0));
        for (t, b) in std::iter::zip(ts, bs) {
            if t0 != t && t0 != b {
                (tt, tb) = (None, None);
            }
            if b0 != t && b0 != b {
                (bt, bb) = (None, None)
            }
            if t == b {
                continue;
            }
            if t == t0 {
                tb = tb.map(|k| k + 1);
            }
            if t == b0 {
                bb = bb.map(|k| k + 1);
            }
            if b == t0 {
                tt = tt.map(|k| k + 1);
            }
            if b == b0 {
                bt = bt.map(|k| k + 1);
            }
        }
        [tt, tb, bt, bb].into_iter().flatten().min().unwrap_or(-1)
    }
}


