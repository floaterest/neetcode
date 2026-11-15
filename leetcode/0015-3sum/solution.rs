impl Solution {
    /// return all distinct triples from xs with sum 0
    pub fn three_sum(mut xs: Vec<i32>) -> Vec<Vec<i32>> {
        xs.sort();
        // for what follows, a<=b<=c are candidates
        let mut res = Vec::with_capacity(xs.len());
        // fix a
        for (i, &a) in xs.iter().enumerate() {
            if a > 0 {
                // sum can't be 0 anymore
                break;
            }
            // skip dups of a
            if i > 0 && xs[i - 1] == a {
                continue;
            }
            // look for b,c such that b+c == a
            let (mut l, mut r) = (i + 1, xs.len() - 1);
            while l < r {
                let (b, c) = (xs[l], xs[r]);
                let s = a + b + c;
                if s < 0 {
                    l += 1;
                } else if s > 0 {
                    r -= 1;
                } else {
                    res.push(vec![a, b, c]);
                    r -= 1;
                    l += 1;
                    while xs[l] == xs[l - 1] && l < r {
                        l += 1;
                    }
                }
            }
        }
        res
    }
}

