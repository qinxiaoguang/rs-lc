use super::Solution;

impl Solution {
    // 是一道搜索题，但搜索的时候可以优化，优化思路就是，每个连通的方格的下一步，只往右或者下走
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut can = vec![vec![true; n as usize]; m as usize];
        for i in 0..m as usize {
            for j in 0..n as usize {
                let (mut ti, mut tj) = (i, j);
                let mut sum = 0;
                while ti != 0 {
                    sum += (ti % 10);
                    ti /= 10;
                }
                while tj != 0 {
                    sum += (tj % 10);
                    tj /= 10;
                }
                can[i][j] = (sum <= k as usize);
            }
        }
        // 从i,j开始搞, 使用宽搜
        let mut q = std::collections::VecDeque::new();
        q.push_back((0usize, 0usize));
        let mut res = 0;
        while let Some((i, j)) = q.pop_front() {
            if i < m as usize && j < n as usize && can[i][j] {
                can[i][j] = false;
                res += 1;
                // 往下移动
                q.push_back((i + 1, j));
                q.push_back((i, j + 1));
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j013() {
        assert_eq!(Solution::moving_count(2, 3, 1), 3);
        assert_eq!(Solution::moving_count(3, 1, 0), 1);
    }
}
