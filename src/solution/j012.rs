use super::Solution;

impl Solution {
    // 查找是否存在某个单词，只要找到单词的起点，并向下dfs即可
    // 但这样的时间复杂度会特别高,好像也没啥别的解了
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let words: Vec<char> = word.chars().collect();
        if words.len() == 0 {
            return false;
        }
        let mut used = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == words[0] {
                    used[i][j] = true;
                    if Self::j012_dfs(&board, 1, &words, (i as i32, j as i32), &mut used) {
                        return true;
                    }
                    used[i][j] = false;
                }
            }
        }
        return false;
    }

    fn j012_dfs(
        board: &Vec<Vec<char>>,
        word_index: usize,
        words: &[char],
        (i, j): (i32, i32),
        used: &mut Vec<Vec<bool>>,
    ) -> bool {
        // word_index成长到len,则说明前边已经找到了，直接返回即可
        if word_index >= words.len() {
            return true;
        }

        // 从当前点开始找,找下一个点是words[word_index]的单词
        // 分为四个方向，上下左右
        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (x, y) in dirs {
            let (ni, nj) = (i + x, j + y);
            if ni < 0
                || nj < 0
                || ni as usize == board.len()
                || nj as usize == board[0].len()
                || used[ni as usize][nj as usize]
            {
                continue;
            }
            let c = board[ni as usize][nj as usize];
            if c == words[word_index] {
                // 下一个方向符合条件
                used[ni as usize][nj as usize] = true;
                if Self::j012_dfs(board, word_index + 1, words, (ni, nj), used) {
                    return true;
                }
                used[ni as usize][nj as usize] = false;
            }
        }

        return false;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j012() {
        assert!(Solution::exist(
            matrix![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'C', 'S'],
                ['A', 'D', 'E', 'E']
            ],
            s!("ABCCED"),
        ));

        assert!(!Solution::exist(
            matrix![['a', 'b'], ['c', 'd']],
            s!("abcd")
        ));
    }
}
