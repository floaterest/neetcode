impl Solution {
    pub fn min_sum(xs: Vec<i32>, ys: Vec<i32>) -> i64 {
        let xs: Vec<i64> = xs.into_iter().map(|x| x.into()).collect();
        let ys: Vec<i64> = ys.into_iter().map(|y| y.into()).collect();
        let (sx, sy): (i64, i64) = (xs.iter().sum(), ys.iter().sum());
        let (cx, cy) = (
            xs.into_iter().filter(|&x| x == 0).count() as i64,
            ys.into_iter().filter(|&y| y == 0).count() as i64,
        );
        let (sx, sy) = (sx + cx, sy + cy);
        match (cx, cy) {
            (0, 0) => if sx == sy { sx } else { -1 },
            (0, _) => if sy <= sx { sx } else { -1 },
            (_, 0) => if sx <= sy { sy } else { -1 },
            (_, _) => sx.max(sy),
        }
    }
}


