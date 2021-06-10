use super::Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 宽搜， 且搜的过程，一旦发现结果已经遍历过了，就不继续搜
        // 用深搜也可以解，但深搜需要剪枝
        let mut queue = std::collections::VecDeque::new();
        let (rows, cols) = (mat.len(), mat[0].len());
        let mut res = vec![vec![i32::MAX; mat[0].len()]; mat.len()];
        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] == 0 {
                    // 前两个是坐标，，第三个是deep
                    queue.push_back((i, j, 0i32));
                    break;
                }
            }
        }
        let dirs = [(0, -1), (-1, 0), (0, 1), (1, 0)];
        while let Some(top) = queue.pop_front() {
            let (x, y, deep) = top;
            if res[x][y] <= deep {
                continue;
            }
            // 结果大于deep， 需要更新结果
            res[x][y] = deep;
            // 四个方向入队列
            for (add_x, add_y) in dirs.iter() {
                let (next_x, next_y) = (x as i32 + add_x, y as i32 + add_y);
                if next_x < 0 || next_y < 0 || next_x >= rows as i32 || next_y >= cols as i32 {
                    continue;
                }
                queue.push_back((
                    next_x as usize,
                    next_y as usize,
                    if mat[next_x as usize][next_y as usize] == 0 {
                        0
                    } else {
                        deep + 1
                    },
                ));
            }
        }

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l542() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
        assert_eq!(
            Solution::update_matrix(vec![
                vec![1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
                vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
                vec![0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
                vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
                vec![0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
                vec![0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
                vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
                vec![1, 1, 1, 1, 0, 1, 0, 0, 1, 1]
            ]),
            vec![
                vec![1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
                vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
                vec![0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
                vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
                vec![0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
                vec![0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
                vec![1, 0, 0, 0, 1, 2, 1, 1, 0, 1],
                vec![2, 1, 1, 1, 1, 2, 1, 0, 1, 0],
                vec![3, 2, 2, 1, 0, 1, 0, 0, 1, 1]
            ]
        );
    }
}
