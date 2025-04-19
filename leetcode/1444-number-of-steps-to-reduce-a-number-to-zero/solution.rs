impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        // convert to bin
        // length (as string) + num of '1'
        let s = format!("{num:b}");
        (s.len() + s.chars().filter(|&c| c == '1').count()) as i32 - 1
    }
}

