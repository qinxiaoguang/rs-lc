use super::Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        // 一看就是dp题
        // 用两个数组保存，一个数组保存当前积中的负数最小值dpmin，一个保存当前积中的正数最大值dpmax
        // 那么结果就是res[i][j] = max(dpmin[i-1][j]*grid[i][j], dpmax[i-1][j]*grid[i][j])
        //          res[i][j] = max(res, dpmin[i][j-1]*grid[i][j], dpmax[i][j-1]*grid[i][j])
        let MOD = 1000000007;
        let row = grid.len();
        let col = grid[0].len();
        let mut dpmin = vec![vec![0i64; col]; row];
        let mut dpmax = vec![vec![0i64; col]; row];
        dpmin[0][0] = grid[0][0] as i64;
        dpmax[0][0] = grid[0][0] as i64;
        for i in 1..row {
            dpmin[i][0] = dpmin[i - 1][0] * grid[i][0] as i64;
            dpmax[i][0] = dpmax[i - 1][0] * grid[i][0] as i64;
        }
        for j in 1..col {
            dpmin[0][j] = dpmin[0][j - 1] * grid[0][j] as i64;
            dpmax[0][j] = dpmax[0][j - 1] * grid[0][j] as i64;
        }
        for i in 1..row {
            for j in 1..col {
                if grid[i][j] < 0 {
                    dpmax[i][j] = dpmin[i - 1][j].min(dpmin[i][j - 1]) * grid[i][j] as i64;
                    dpmin[i][j] = dpmax[i - 1][j].max(dpmax[i][j - 1]) * grid[i][j] as i64;
                } else {
                    dpmax[i][j] = dpmax[i - 1][j].max(dpmax[i][j - 1]) * grid[i][j] as i64;
                    dpmin[i][j] = dpmin[i - 1][j].min(dpmin[i][j - 1]) * grid[i][j] as i64;
                }
            }
        }
        if dpmax[row - 1][col - 1] < 0 {
            -1
        } else {
            (dpmax[row - 1][col - 1] % MOD) as i32
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1594() {
        assert_eq!(
            Solution::max_product_path(vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2],]),
            -1
        );
        assert_eq!(
            Solution::max_product_path(vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1],]),
            8
        );
        assert_eq!(
            Solution::max_product_path(vec![vec![1, 3], vec![0, -4],]),
            0
        );
        assert_eq!(
            Solution::max_product_path(vec![
                vec![1, 4, 4, 0],
                vec![-2, 0, 0, 1],
                vec![1, -1, 1, 1],
            ]),
            2
        );
    }
}
