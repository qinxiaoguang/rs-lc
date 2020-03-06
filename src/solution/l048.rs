pub struct Solution {}

impl Solution {
    // 将一个matrix 旋转,在原地旋转，空间复杂度O(1)
    // 其实就是 对角线旋转一次，再左右/上下旋转一次就行了
    // 1. 左斜对焦旋转 -> 左右旋转
    // 2. 右斜对角旋转 -> 上下旋转
    // 任选其一
    // 对角线旋转方法是，判断i-j是否大于0
    #![allow(dead_code)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // 对角线旋转
        let n = matrix.len();
        for i in 0..n {
            for j in 0..n {
                if j > i {
                    let tmp = matrix[i][j];
                    matrix[i][j] = matrix[j][i];
                    matrix[j][i] = tmp;
                }
            }
        }

        // 左右旋转
        for i in 0..n {
            for j in 0..n {
                if j < n / 2 {
                    matrix[i].swap(j, n - j - 1);
                }
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l048() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        println!("{:?}", matrix);
    }
}
