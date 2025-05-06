impl Solution {
    fn f(xs: &[i32]) -> i32 {
        if xs.len() <= 1 {
            return 0;
        }
        let (a, b) = (xs.first().unwrap(), xs.last().unwrap());
        Self::f(if a <= b {
            &xs[1..]
        } else {
            &xs[..xs.len() - 1]
        })
        .max((xs.len() as i32 - 1) * a.min(b))
    }
    pub fn max_area(xs: Vec<i32>) -> i32 {
        Self::f(&xs)
    }
}


