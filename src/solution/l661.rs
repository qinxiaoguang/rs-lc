pub struct Solution {}

/*
 * @lc app=leetcode.cn id=661 lang=rust
 *
 * [661] 图片平滑器
 *
 * https://leetcode-cn.com/problems/image-smoother/description/
 *
 * algorithms
 * Easy (52.75%)
 * Likes:    55
 * Dislikes: 0
 * Total Accepted:    8.8K
 * Total Submissions: 16.4K
 * Testcase Example:  '[[1,1,1],[1,0,1],[1,1,1]]'
 *
 * 包含整数的二维矩阵 M 表示一个图片的灰度。你需要设计一个平滑器来让每一个单元的灰度成为平均灰度 (向下舍入)
 * ，平均灰度的计算是周围的8个单元和它本身的值求平均，如果周围的单元格不足八个，则尽可能多的利用它们。
 *
 * 示例 1:
 *
 *
 * 输入:
 * [[1,1,1],
 * ⁠[1,0,1],
 * ⁠[1,1,1]]
 * 输出:
 * [[0, 0, 0],
 * ⁠[0, 0, 0],
 * ⁠[0, 0, 0]]
 * 解释:
 * 对于点 (0,0), (0,2), (2,0), (2,2): 平均(3/4) = 平均(0.75) = 0
 * 对于点 (0,1), (1,0), (1,2), (2,1): 平均(5/6) = 平均(0.83333333) = 0
 * 对于点 (1,1): 平均(8/9) = 平均(0.88888889) = 0
 *
 *
 * 注意:
 *
 *
 * 给定矩阵中的整数范围为 [0, 255]。
 * 矩阵的长和宽的范围均为 [1, 150]。
 *
 *
 */

// @lc code=start
impl Solution {
    // 和289题一个样，为了方便，还是复制个数组得了
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let src = m.clone();
        let mut m = m;
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
        if m.len() == 0 {
            return m;
        }
        let x_len = m.len() as i32;
        let y_len = m[0].len() as i32;
        for i in 0..x_len {
            for j in 0..y_len {
                let mut sum = src[i as usize][j as usize];
                let mut cnt = 1;
                for (addx, addy) in &eight_dir {
                    let (x, y) = (i + addx, j + addy);
                    if x < 0 || x >= x_len || y < 0 || y >= y_len {
                        continue;
                    }
                    sum += src[x as usize][y as usize];
                    cnt += 1;
                }
                m[i as usize][j as usize] = sum / cnt as i32;
            }
        }
        m
    }

    fn check(x: i32, y: i32, x_len: i32, y_len: i32) -> bool {
        !(x < 0 || x >= x_len || y < 0 || y >= y_len)
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l661() {}
}
