impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let (mut ibad, mut imin, mut imax) = (None, None, None);
        // ibad is latest index where num is <m and >M
        // imin is latest index where num is =m
        // imax is latest index where num is =M
        // thus (i, j) is valid iff ibad < i <= min(imin, imax)
        nums.into_iter()
            .enumerate()
            .map(|(j, x)| {
                (x == min_k).then(|| imin = Some(j));
                (x == max_k).then(|| imax = Some(j));
                (x < min_k || x > max_k).then(|| ibad = Some(j));
                // return number of valid subarrays ending at j
                if let Some(i) = imin.zip(imax).map(|(i1, i2)| i1.min(i2)) {
                    if let Some(ib) = ibad {
                        // i < ibad => 0
                        // ibad < i => i - ibad
                        i.checked_sub(ib).unwrap_or(0)
                    } else {
                        // ibad == None => i + 1
                        i + 1
                    }
                } else {
                    // imin or imax is None => 0
                    0
                }
            })
            .sum::<usize>() as i64
    }
}


