class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        row, col, squ = [0]*9, [0]*9,[0]*9
        for i, j, c in [(i,j,board[i][j]) for i in range(9) for j in range(9)]:
            if c == '.':
                continue
            v = 1 << (int(c)-1)
            s = (i//3)*3+(j//3)
            if (row[i] | col[j] | squ[s]) & v:
                return False
            row[i] |= v
            col[j] |= v
            squ[s] |= v

        return True