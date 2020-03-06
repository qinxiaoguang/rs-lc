pub struct Solution {}

impl Solution {
    // 给定n, 求1~n^2,螺旋生成的数组
    #![allow(dead_code)]
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n <= 0 {
            return vec![];
        }
        let mut matrix = vec![vec![-1_i32; n as usize]; n as usize];
        let (mut x, mut y) = (0_usize, 0_usize);
        enum Dir {
            Up,
            Down,
            Left,
            Right,
        }
        let mut dir = Dir::Right;
        let mut step = 1_i32;
        loop {
            matrix[x][y] = step;
            step += 1;
            if step > n * n {
                break;
            }
            loop {
                match dir {
                    Dir::Up => {
                        if x <= 0 || matrix[x - 1][y] != -1 {
                            dir = Dir::Right;
                            continue;
                        }
                        x -= 1;
                        break;
                    }
                    Dir::Right => {
                        if y >= n as usize - 1 || matrix[x][y + 1] != -1 {
                            dir = Dir::Down;
                            continue;
                        }
                        y += 1;
                        break;
                    }
                    Dir::Down => {
                        if x >= n as usize - 1 || matrix[x + 1][y] != -1 {
                            dir = Dir::Left;
                            continue;
                        }
                        x += 1;
                        break;
                    }
                    Dir::Left => {
                        if y <= 0 || matrix[x][y - 1] != -1 {
                            dir = Dir::Up;
                            continue;
                        }
                        y -= 1;
                        break;
                    }
                }
            }
        }
        matrix
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l059() {
        println!("{:?}", Solution::generate_matrix(0));
        println!("{:?}", Solution::generate_matrix(1));
        println!("{:?}", Solution::generate_matrix(3));
        println!("{:?}", Solution::generate_matrix(4));
    }
}
