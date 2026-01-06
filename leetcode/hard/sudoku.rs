impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {

        fn is_safe(board: &Vec<Vec<char>>, row: usize, col: usize, dig: char) -> bool {
            for r in 0..9 {
                if board[r][col] == dig {
                    return false;
                }
            }

            for c in 0..9 {
                if board[row][c] == dig {
                    return false;
                }
            }

            let sr = (row / 3) * 3;
            let sc = (col / 3) * 3;

            for i in sr..sr + 3 {
                for j in sc..sc + 3 {
                    if board[i][j] == dig {
                        return false;
                    }
                }
            }

            true
        }

        fn solve(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
            if row == 9 {
                return true;
            }

            let mut next_row = row;
            let mut next_col = col + 1;

            if col == 8 {
                next_row = row + 1;
                next_col = 0;
            }

            if board[row][col] != '.' {
                return solve(board, next_row, next_col);
            }

            for dig in '1'..='9' {
                if is_safe(board, row, col, dig) {
                    board[row][col] = dig;

                    if solve(board, next_row, next_col) {
                        return true;
                    }

                    board[row][col] = '.';
                }
            }

            false
        }

        solve(board, 0, 0);
    }
}
