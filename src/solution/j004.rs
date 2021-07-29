use super::Solution;

impl Solution {
    // 重点是要从右上角开始查找，每次查找需要判断往左移动一格还是往下移动一格
    // 如果目标值比当前值小，则往左移动, 如果目标值比当前值大，则往下移(因为往左移没意义,往右移则不满足，因为当前坐标本身就是从右边移动过来的)
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        }
        let (mut i, mut j) = (0, matrix[0].len() - 1);
        while matrix[i][j] != target {
            if matrix[i][j] > target {
                // 大于
                if j != 0 {
                    j -= 1;
                    continue;
                }
                return false;
            } else {
                if i != matrix.len() - 1 {
                    i += 1;
                    continue;
                }
                return false;
            }
        }
        return true;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j004() {
        assert_eq!(
            Solution::find_number_in2_d_array(
                matrix![
                    [1, 4, 7, 11, 15],
                    [2, 5, 8, 12, 19],
                    [3, 6, 9, 16, 22],
                    [10, 13, 14, 17, 24],
                    [18, 21, 23, 26, 30]
                ],
                5
            ),
            true
        );
        assert_eq!(
            Solution::find_number_in2_d_array(
                matrix![
                    [1, 4, 7, 11, 15],
                    [2, 5, 8, 12, 19],
                    [3, 6, 9, 16, 22],
                    [10, 13, 14, 17, 24],
                    [18, 21, 23, 26, 30]
                ],
                20
            ),
            false
        );
    }
}
