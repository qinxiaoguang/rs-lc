pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 单词在图中能否连到一起
    // 使用dfs + 剪枝
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() {
            return false;
        }
        let m = board.len();
        let n = board[0].len();
        let mut board = board;
        for i in 0..m {
            for j in 0..n {
                if Self::dfs(&mut board, &mut vec![vec![false; n]; m], &word, 0, (i, j)) {
                    return true;
                }
            }
        }
        false
    }
    fn dfs(
        board: &Vec<Vec<char>>,
        used: &mut Vec<Vec<bool>>,
        word: &str,
        now_index: usize,
        (x, y): (usize, usize),
    ) -> bool {
        if used[x][y] {
            return false;
        }
        used[x][y] = true;
        if board[x][y] == word.as_bytes()[now_index] as char {
            if now_index + 1 == word.len() {
                return true;
            }
            let m = board.len();
            let n = board[0].len();
            let dir = vec![(-1_i32, 0_i32), (1, 0), (0, -1), (0, 1)];
            for (dir_x, dir_y) in dir {
                // 四个方向
                let (i, j) = (x as i32 + dir_x, y as i32 + dir_y);
                if i < 0 || i >= m as i32 || j < 0 || j >= n as i32 {
                    continue;
                }
                let (i, j) = (i as usize, j as usize);
                if Self::dfs(board, used, word, now_index + 1, (i, j)) {
                    return true;
                }
            }
        }
        used[x][y] = false;
        return false;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l079() {
        println!(
            "{}",
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCEE".to_string()
            )
        );
        println!(
            "{}",
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_string()
            )
        );
    }
}
