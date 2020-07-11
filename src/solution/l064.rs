pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 找出从左上到右小的最小的路径和
    // 使用dp的思路，与63/62题基本一样
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        if grid.len() == 0 {
            return 0;
        }
        let (m, n) = (grid.len(), grid[0].len());
        if n == 0 {
            return 0;
        }
        for i in 0..m as usize {
            for j in 0..n as usize {
                if i == 0 && j == 0 {
                    continue;
                } else if i == 0 {
                    grid[i][j] += grid[i][j - 1];
                } else if j == 0 {
                    grid[i][j] += grid[i - 1][j];
                } else {
                    grid[i][j] += std::cmp::min(grid[i - 1][j], grid[i][j - 1]);
                }
            }
        }
        println!("{:?}", grid);
        grid[m - 1][n - 1]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l064() {
        println!(
            "{}",
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
    }
}
