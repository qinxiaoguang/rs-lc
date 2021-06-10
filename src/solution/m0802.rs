use super::Solution;

impl Solution {
    // 只是返回一条可行的路径, 但是r,c值不超过100， 采用dfs或bfs会超时，2^N次方的时间复杂度
    // 所以考虑使用dp来解，其中dp[i][j]记录其前置节点,最后从dp[last][last]一个个往前找即可
    // 若dp[i][j] == (-1,-1)表示不可达
    pub fn path_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r = obstacle_grid.len();
        if r == 0 {
            return vec![];
        }
        let c = obstacle_grid[0].len();
        if c == 0 {
            return vec![];
        }
        if obstacle_grid[0][0] == 1 || obstacle_grid[r - 1][c - 1] == 1 {
            return vec![];
        }
        let mut dp = vec![vec![(-1, -1); c]; r];
        dp[0][0] = (-2, -2);
        for i in 0..r {
            for j in 0..c {
                if obstacle_grid[i][j] == 1 {
                    continue;
                }
                if i != 0 && dp[i - 1][j].0 != -1 {
                    // 前置不行
                    // 前置可达
                    dp[i][j] = (i as i32 - 1, j as i32);
                    continue;
                }
                if j != 0 && dp[i][j - 1].0 != -1 {
                    dp[i][j] = (i as i32, j as i32 - 1);
                }
            }
        }
        let (mut x, mut y) = (r - 1, c - 1);
        let mut res = vec![];
        loop {
            if dp[x][y].0 == -1 {
                return vec![];
            }
            if dp[x][y].0 == -2 {
                // 到达起点
                res.push(vec![0, 0]);
                res.reverse();
                return res;
            }
            res.push(vec![x as i32, y as i32]);
            let tmp = (dp[x][y].0 as usize, dp[x][y].1 as usize);
            x = tmp.0;
            y = tmp.1;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_m0802() {
        let res = Solution::path_with_obstacles(matrix![[0, 0, 0], [0, 1, 0], [0, 0, 0]]);
        println!("res = {:?}", res);
        let res = Solution::path_with_obstacles(matrix![[1]]);
        println!("res = {:?}", res);
        let res = Solution::path_with_obstacles(matrix![[0]]);
        println!("res = {:?}", res);
        let res = Solution::path_with_obstacles(matrix![[0, 1]]);
        println!("res = {:?}", res);
        let res = Solution::path_with_obstacles(matrix![[0, 1], [0, 0]]);
        println!("res = {:?}", res);
    }
}
