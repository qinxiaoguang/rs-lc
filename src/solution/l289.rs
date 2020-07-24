pub struct Solution {}

/*
 * @lc app=leetcode.cn id=289 lang=rust
 *
 * [289] 生命游戏
 *
 * https://leetcode-cn.com/problems/game-of-life/description/
 *
 * algorithms
 * Medium (69.65%)
 * Likes:    232
 * Dislikes: 0
 * Total Accepted:    36.1K
 * Total Submissions: 48.5K
 * Testcase Example:  '[[0,1,0],[0,0,1],[1,1,1],[0,0,0]]'
 *
 * 根据 百度百科 ，生命游戏，简称为生命，是英国数学家约翰·何顿·康威在 1970 年发明的细胞自动机。
 *
 * 给定一个包含 m × n 个格子的面板，每一个格子都可以看成是一个细胞。每个细胞都具有一个初始状态：1 即为活细胞（live），或 0
 * 即为死细胞（dead）。每个细胞与其八个相邻位置（水平，垂直，对角线）的细胞都遵循以下四条生存定律：
 *
 *
 * 如果活细胞周围八个位置的活细胞数少于两个，则该位置活细胞死亡；
 * 如果活细胞周围八个位置有两个或三个活细胞，则该位置活细胞仍然存活；
 * 如果活细胞周围八个位置有超过三个活细胞，则该位置活细胞死亡；
 * 如果死细胞周围正好有三个活细胞，则该位置死细胞复活；
 *
 *
 *
 * 根据当前状态，写一个函数来计算面板上所有细胞的下一个（一次更新后的）状态。下一个状态是通过将上述规则同时应用于当前状态下的每个细胞所形成的，其中细胞的出生和死亡是同时发生的。
 *
 *
 *
 * 示例：
 *
 * 输入：
 * [
 * [0,1,0],
 * [0,0,1],
 * [1,1,1],
 * [0,0,0]
 * ]
 * 输出：
 * [
 * [0,0,0],
 * [1,0,1],
 * [0,1,1],
 * [0,1,0]
 * ]
 *
 *
 *
 * 进阶：
 *
 *
 * 你可以使用原地算法解决本题吗？请注意，面板上所有格子需要同时被更新：你不能先更新某些格子，然后使用它们的更新后的值再更新其他格子。
 * 本题中，我们使用二维数组来表示面板。原则上，面板是无限的，但当活细胞侵占了面板边界时会造成问题。你将如何解决这些问题？
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        //下一个状态，依赖于上个状态，所以需要额外的空间来保存上一个状态
        // 但是本题要求使用原地算法解决
        // 此时就要使用状态了。该题有两个状态，1是活着，0是死的
        // 而我们可以增加状态，如2表示，之前是活的，现在是死的。
        // 而3表示之前是死的，现在是活的。利用状态，来临时保存细胞上次的状态
        // 要点就是增加状态
        if board.len() == 0 {
            return;
        }
        let eight_dir: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let x_len = board.len() as i32;
        let y_len = board[0].len() as i32;
        for i in 0..x_len {
            for j in 0..y_len {
                // 判断8个方向
                let mut live_cnt = 0;
                for (addx, addy) in &eight_dir {
                    let (x, y) = (i + addx, j + addy);
                    if x < 0 || x >= x_len || y < 0 || y >= y_len {
                        continue;
                    }
                    let state = board[x as usize][y as usize];
                    live_cnt += if state == 1 || state == 2 { 1 } else { 0 };
                }

                let state = board[i as usize][j as usize];
                if (live_cnt == 2 && state == 1) || live_cnt == 3 {
                    // 细胞复活
                    board[i as usize][j as usize] = if state == 0 { 3 } else { 1 }
                } else {
                    // 细胞死亡
                    board[i as usize][j as usize] = if state == 1 { 2 } else { 0 }
                }
            }
        }
        for i in 0..x_len as usize {
            for j in 0..y_len as usize {
                let tmp = board[i][j];
                board[i][j] = match tmp {
                    2 => 0,
                    3 => 1,
                    _ => tmp,
                };
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l289() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        let res = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(res, board);
    }
}
