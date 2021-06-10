use super::Solution;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        // 宽搜，问题是怎么写
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(board.clone());
        assert!(set.contains(&board));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((board, 0));
        let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        while let Some((b, deep)) = queue.pop_front() {
            // 0的位置
            let (mut x, mut y) = (0i32, 0i32);
            // 判断是否合法，并确定0的位置
            let mut find = true;
            for r in 0..2 {
                for c in 0..3 {
                    if b[r][c] == 0 {
                        x = r as i32;
                        y = c as i32;
                    } else if r * 3 + c + 1 != b[r][c] as usize {
                        find = false;
                    }
                }
            }
            if find {
                return deep;
            }
            // 0 可以往上下左右四个方向走
            for (r, c) in dirs.iter() {
                let (next_x, next_y) = (x + r, y + c);
                if next_x >= 0 && next_x < 2 && next_y >= 0 && next_y < 3 {
                    let mut new_b = b.clone();
                    new_b[x as usize][y as usize] = new_b[next_x as usize][next_y as usize];
                    new_b[next_x as usize][next_y as usize] = 0;
                    if !set.contains(&new_b) {
                        set.insert(new_b.clone());
                        queue.push_back((new_b, deep + 1));
                    }
                }
            }
        }

        -1i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l773() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]),
            5
        )
    }
}
