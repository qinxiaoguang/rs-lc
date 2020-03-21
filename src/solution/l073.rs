pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 原地修改数组，若[i,j]处的数为0，则将该i行j列都置为0
    // 方法是先遍历一遍数组，若[i][j]为0 ，则将[0][j]、[i][0]置为0
    // 将最边的数做为判断条件即可。
    // 但是第二次遍历的时候，要倒序。自己想想为什么
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (mut left, mut top) = (false, false);
        if matrix.is_empty() {
            return;
        }
        if matrix[0].is_empty() {
            return;
        }
        for j in 0..matrix[0].len() {
            if matrix[0][j] == 0 {
                top = true;
            }
        }
        for i in 0..matrix.len() {
            if matrix[i][0] == 0 {
                left = true;
            }
        }
        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }
        for i in (1..matrix.len()).rev() {
            for j in (1..matrix[0].len()).rev() {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if left {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
        }
        if top {
            for j in 0..matrix[0].len() {
                matrix[0][j] = 0;
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l073() {
        let mut v = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut v);
        println!("{:?}", v);

        let mut v = vec![vec![1, 1, 1], vec![0, 1, 2]];
        Solution::set_zeroes(&mut v);
        println!("{:?}", v);
    }
}
