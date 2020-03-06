pub struct Solution {}

impl Solution {
    // 给定一个包含 m x n 个元素的矩阵（m 行, n 列），请按照顺时针螺旋顺序，返回矩阵中的所有元素。
    // 其实就是模拟打印
    #[allow(dead_code)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        if matrix.len() == 0 {
            return res;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut used = vec![vec![false; n]; m];
        let (mut x, mut y) = (0_usize, 0_usize);
        enum Dir {
            Up,
            Down,
            Left,
            Right,
        }
        let mut dir = Dir::Right;
        loop {
            res.push(matrix[x][y]);
            used[x][y] = true;
            if res.len() == m * n {
                break;
            }
            loop {
                match dir {
                    Dir::Up => {
                        if x <= 0 || used[x - 1][y] {
                            dir = Dir::Right;
                            continue;
                        }
                        x -= 1;
                        break;
                    }
                    Dir::Right => {
                        if y >= n - 1 || used[x][y + 1] {
                            dir = Dir::Down;
                            continue;
                        }
                        y += 1;
                        break;
                    }
                    Dir::Down => {
                        if x >= m - 1 || used[x + 1][y] {
                            dir = Dir::Left;
                            continue;
                        }
                        x += 1;
                        break;
                    }
                    Dir::Left => {
                        if y <= 0 || used[x][y - 1] {
                            dir = Dir::Up;
                            continue;
                        }
                        y -= 1;
                        break;
                    }
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l054() {
        println!(
            "{:?}",
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],])
        );

        println!(
            "{:?}",
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
            ])
        );
    }
}
