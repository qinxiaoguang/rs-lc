use super::Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        // 投影，只需要用一个数组保存一个方向的投影，其他方向的投影可以在遍历的时候获取结果
        let mut res = 0;
        let mut cols = vec![0; grid.len()]; // 竖直方向投影，某列的最大值
        for i in 0..grid.len() {
            let mut row = 0;
            for j in 0..grid[0].len() {
                if grid[i][j] > 0 {
                    cols[j] = cols[j].max(grid[i][j]);
                    row = row.max(grid[i][j]);
                    res += 1;
                }
            }
            res += row;
        }
        res + cols.iter().sum::<i32>()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l883() {
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    }
}
