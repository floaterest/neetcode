impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let (mut row, mut col, mut squ) = ([0; 9], [0; 9], [0; 9]);
        // nth bit of row[i] denotes if n is seen at ith row
        let iter = board.into_iter().enumerate().flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .filter(|&(_, c)| c != '.')
                .map(move |(j, ch)| (i, j, (i / 3) * 3 + j / 3, ch))
        });
        for (i, j, s, ch) in iter {
            let v = 1 << (ch as u32 - '1' as u32);
            if (row[i] | col[j] | squ[s]) & v > 0 {
                return false;
            }
            row[i] |= v;
            col[j] |= v;
            squ[s] |= v;
        }
        true
    }
}
