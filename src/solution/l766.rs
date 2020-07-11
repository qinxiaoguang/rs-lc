pub struct Solution {}

/*
 * @lc app=leetcode.cn id=766 lang=rust
 *
 * [766] 托普利茨矩阵
 *
 * https://leetcode-cn.com/problems/toeplitz-matrix/description/
 *
 * algorithms
 * Easy (64.19%)
 * Likes:    116
 * Dislikes: 0
 * Total Accepted:    14.6K
 * Total Submissions: 22K
 * Testcase Example:  '[[1,2,3,4],[5,1,2,3],[9,5,1,2]]'
 *
 * 如果一个矩阵的每一方向由左上到右下的对角线上具有相同元素，那么这个矩阵是托普利茨矩阵。
 *
 * 给定一个 M x N 的矩阵，当且仅当它是托普利茨矩阵时返回 True。
 *
 * 示例 1:
 *
 * 输入:
 * matrix = [
 * [1,2,3,4],
 * [5,1,2,3],
 * [9,5,1,2]
 * ]
 * 输出: True
 * 解释:
 * 在上述矩阵中, 其对角线为:
 * "[9]", "[5, 5]", "[1, 1, 1]", "[2, 2, 2]", "[3, 3]", "[4]"。
 * 各条对角线上的所有元素均相同, 因此答案是True。
 *
 *
 * 示例 2:
 *
 * 输入:
 * matrix = [
 * [1,2],
 * [2,2]
 * ]
 * 输出: False
 * 解释:
 * 对角线"[1, 2]"上的元素不同。
 *
 *
 * 说明:
 *
 *
 * matrix 是一个包含整数的二维数组。
 * matrix 的行数和列数均在 [1, 20]范围内。
 * matrix[i][j] 包含的整数在 [0, 99]范围内。
 *
 *
 * 进阶:
 *
 *
 * 如果矩阵存储在磁盘上，并且磁盘内存是有限的，因此一次最多只能将一行矩阵加载到内存中，该怎么办？
 * 如果矩阵太大以至于只能一次将部分行加载到内存中，该怎么办？
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        // 先从纵坐标，(0,len-1) 往后遍历 到(0,0)
        // 再从横坐标, (0,1)往后遍历到(0, len-1)
        let x_len = matrix.len();
        if x_len == 0 {
            return true;
        }
        let y_len = matrix[0].len();
        if y_len == 0 {
            return true;
        }

        if (0..(y_len - 1)).any(|y| !Self::is_right((0, y as i32, x_len, y_len), &matrix))
            || (1..(x_len - 1)).any(|x| !Self::is_right((x as i32, 0, x_len, y_len), &matrix))
        {
            return false;
        }

        true
    }

    // 检测是否越界
    pub fn check((x, y, max_x, max_y): (i32, i32, usize, usize)) -> bool {
        x < max_x as i32 && y < max_y as i32
    }

    // 判断(x,y)的斜对角是否都相等
    pub fn is_right(
        (mut x, mut y, max_x, max_y): (i32, i32, usize, usize),
        matrix: &Vec<Vec<i32>>,
    ) -> bool {
        loop {
            let (next_x, next_y) = (x + 1, y + 1);
            if Self::check((next_x, next_y, max_x, max_y)) {
                if matrix[x as usize][y as usize] != matrix[next_x as usize][next_y as usize] {
                    return false;
                }
                x += 1;
                y += 1;
            } else {
                break;
            }
        }
        return true;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l766() {
        assert!(Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2]
        ]));

        assert!(!Solution::is_toeplitz_matrix(vec![
            vec![2, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2]
        ]));

        assert!(Solution::is_toeplitz_matrix(vec![]));
    }
}
