use super::Solution;

impl Solution {
    // 解数独
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_used = [[false; 9]; 9]; // 行
        let mut col_used = [[false; 9]; 9]; // 列
        let mut block_used = [[false; 9]; 9]; // 块

        for row in 0..9 {
            for col in 0..9 {
                let c = board[row][col];
                if c == '.' {
                    continue;
                }
                let num = (c as usize - '0' as usize) - 1;
                row_used[row][num] = true;
                col_used[col][num] = true;
                block_used[Self::get_block((row, col))][num] = true;
            }
        }
        Self::l037_dfs(
            board,
            &mut row_used,
            &mut col_used,
            &mut block_used,
            (0, 0),
            &mut false,
        );
    }

    // dfs + 剪枝
    fn l037_dfs(
        boards: &mut Vec<Vec<char>>,
        row_used: &mut [[bool; 9]; 9],
        col_used: &mut [[bool; 9]; 9],
        block_used: &mut [[bool; 9]; 9],
        (row, col): (usize, usize),
        end: &mut bool,
    ) {
        if *end {
            return;
        }
        let (mut next_row, mut next_col) = (0usize, 0usize);
        let mut find = false;
        // 寻找下一个要遍历的点
        for r in row..9 {
            for c in 0..9 {
                if r == row && c < col {
                    continue;
                }
                if boards[r][c] == '.' {
                    next_row = r;
                    next_col = c;
                    find = true;
                    break;
                }
            }
            if find {
                break;
            }
        }
        if !find {
            *end = true;
            return;
        }
        let next_block = Self::get_block((next_row, next_col));
        for num in 0..9 {
            if row_used[next_row][num] || col_used[next_col][num] || block_used[next_block][num] {
                continue;
            }
            row_used[next_row][num] = true;
            col_used[next_col][num] = true;
            block_used[next_block][num] = true;
            boards[next_row][next_col] = ('0' as u8 + num as u8 + 1) as char;
            Self::l037_dfs(
                boards,
                row_used,
                col_used,
                block_used,
                (next_row, next_col),
                end,
            );
            if *end {
                return;
            }
            boards[next_row][next_col] = '.';
            row_used[next_row][num] = false;
            col_used[next_col][num] = false;
            block_used[next_block][num] = false;
        }
    }

    fn get_block((row, col): (usize, usize)) -> usize {
        let (r, c) = (row / 3 + 1, col / 3 + 1);
        (r - 1) * 3 + c - 1
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l037() {
        let mut board = vec![vec!['.'; 9]; 9];
        Solution::solve_sudoku(&mut board);
        println!("res:{:?}", board)
    }
}
