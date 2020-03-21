pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 经典题，搜索二维矩阵，其中矩阵是纵列和横列都是从小到大排序的
    // 找到该矩阵中是存在目标值
    // 从左下角往右上角搞，每次先比较左下角的值是否比目标值大，若大，则最后一行被排除
    // 再比较右上角是否比目标值大，若大，则最后一列排除，依次类推。
    // 二分的方法则为: 将该数组看做一维数组，使用二分查找即可
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        if m == 0 {
            return false;
        }
        let n = matrix[0].len();
        if n == 0 {
            return false;
        }
        let (mut left, mut right) = (0_usize, m as usize * n as usize - 1);
        while left < right {
            let mid = (left + right) / 2;
            let (x, y) = (mid / n, mid % n);
            if matrix[x][y] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        println!("{},{}", left / n, left % n);
        if left / n >= m || left % n >= n {
            return false;
        }
        matrix[left / n][left % n] == target
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l074() {
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                3
            )
        );
        assert_eq!(false, Solution::search_matrix(vec![vec![1]], 2));
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                20
            )
        );
    }
}
