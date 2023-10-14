impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<_> = s.chars().collect();
        let n = s.len();
        let mut dp = vec![vec![true; n]; n];
        (0..n - 1).for_each(|i| dp[i][i + 1] = s[i] == s[i + 1]);
        (0..n.max(2) - 2)
            .rev()
            .flat_map(move |i| (i + 2..n).map(move |j| (i, j)))
            .for_each(|(i, j)| dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1]);
        let (i, j) = dp
            .iter()
            .enumerate()
            .flat_map(|(i, arr)| arr.iter().enumerate().skip(i).map(move |(j, b)| (i, j, b)))
            .filter_map(|(i, j, &b)| if b { Some((i, j)) } else { None })
            .max_by_key(|(i, j)| j - i)
            .unwrap();
        s[i..=j].iter().collect()
    }
}

