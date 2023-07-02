use super::Solution;

struct NumMatrix {
    pre: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    // 初始化
    // 一维数组使用前缀和
    // 二维数组也可以使用前缀和，只不过二维数组的前缀和表示的是从(0,0)到该点的子矩阵的和
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut pre = vec![vec![0; matrix[0].len()]; matrix.len()];
        matrix.iter().enumerate().for_each(|(x, vec)| {
            let mut sum = 0;
            vec.iter().enumerate().for_each(|(y, v)| {
                sum += v;
                pre[x][y] = if x == 0 { 0 } else { pre[x - 1][y] } + sum;
            });
        });
        return NumMatrix { pre: pre };
    }

    // 返回子矩阵和
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        // x - a - b + c;
        self.pre[row2][col2]
            + if row1 == 0 || col1 == 0 {
                0
            } else {
                self.pre[row1 - 1][col1 - 1]
            }
            - if row1 == 0 {
                0
            } else {
                self.pre[row1 - 1][col2]
            }
            - if col1 == 0 {
                0
            } else {
                self.pre[row2][col1 - 1]
            }
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_013() {
        let obj = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);

        assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
        assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
        assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
    }
}
