pub struct Solution {}
/*
 * @lc app=leetcode.cn id=864 lang=rust
 *
 * [864] 获取所有钥匙的最短路径
 *
 * https://leetcode-cn.com/problems/shortest-path-to-get-all-keys/description/
 *
 * algorithms
 * Hard (41.48%)
 * Likes:    48
 * Dislikes: 0
 * Total Accepted:    1.6K
 * Total Submissions: 3.9K
 * Testcase Example:  '["@.a.#","###.#","b.A.B"]'
 *
 * 给定一个二维网格 grid。 "." 代表一个空房间， "#" 代表一堵墙， "@" 是起点，（"a", "b", ...）代表钥匙，（"A",
 * "B", ...）代表锁。
 *
 *
 * 我们从起点开始出发，一次移动是指向四个基本方向之一行走一个单位空间。我们不能在网格外面行走，也无法穿过一堵墙。如果途经一个钥匙，我们就把它捡起来。除非我们手里有对应的钥匙，否则无法通过锁。
 *
 * 假设 K 为钥匙/锁的个数，且满足 1 <= K <= 6，字母表中的前 K
 * 个字母在网格中都有自己对应的一个小写和一个大写字母。换言之，每个锁有唯一对应的钥匙，每个钥匙也有唯一对应的锁。另外，代表钥匙和锁的字母互为大小写并按字母顺序排列。
 *
 * 返回获取所有钥匙所需要的移动的最少次数。如果无法获取所有钥匙，返回 -1 。
 *
 *
 *
 * 示例 1：
 *
 * 输入：["@.a.#","###.#","b.A.B"]
 * 输出：8
 *
 *
 * 示例 2：
 *
 * 输入：["@..aA","..B#.","....b"]
 * 输出：6
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= grid.length <= 30
 * 1 <= grid[0].length <= 30
 * grid[i][j] 只含有 '.', '#', '@', 'a'-'f' 以及 'A'-'F'
 * 钥匙的数目范围是 [1, 6]，每个钥匙都对应一个不同的字母，正好打开一个对应的锁。
 *
 *
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    // 1. 获取钥匙可能不需要经过锁
    // 2. 获取最小步数却可能经过锁
    // 而开锁与不开锁也是一个状态转移过程，所以将该状态加入到bfs中
    // 刚开始用dfs来解的，但是题中没说走过的路不能再走，所以dfs一定是不行的，只能使用bfs来解
    // 用dfs浪费了很多时间。。。。
    // 但是bfs还有一个问题，如果此无解，什么样才是终点？
    // 如果不限制终点，将会超时
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let grid: Vec<Vec<char>> = grid
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();

        let mut all_keys_num = 0;
        let (mut start, mut end) = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    'a'..='z' => {
                        all_keys_num += 1;
                    }
                    '@' => {
                        start = i as i32;
                        end = j as i32;
                    }
                    _ => {}
                }
            }
        }
        /*错误解法
        let mut res = std::i32::MAX;
        let mut used = vec![vec![false; grid[0].len()]; grid.len()];
        used[start as usize][end as usize] = true;
        Self::dfs(
            &grid,
            (start, end),
            &mut vec![false; all_keys_num],
            all_keys_num as i32,
            0,
            0,
            &mut res,
            &mut used,
        );
        if res == std::i32::MAX {
            -1
        } else {
            res
        }*/
        let dirs = vec![(0, 1), (0, -1), (1, 0), (1, -1)];
        let mut step = 0;
        let mut keys = vec![false; all_keys_num];
        let mut queue = vec![];
        let now_keys = 0;
        let mut keys_set = HashSet::new();
        queue.push((start as i32, end as i32, step, keys_set));
        while queue.len() > 0 {
            let (x, y, step, keys_set) = queue.remove(0);
        }
        return -1;
    }

    // 当前遍历到的点
    fn dfs(
        grid: &Vec<Vec<char>>,
        (x, y): (i32, i32),
        keys: &mut Vec<bool>,
        all_keys_num: i32,
        now_keys: i32,
        step: i32,
        min: &mut i32,
        used: &mut Vec<Vec<bool>>,
    ) {
        /*println!(
            " x:{},y:{},keys:{:?},all_keys_num:{},now_keys:{}, step:{},min:{},used:{:?}",
            x, y, keys, all_keys_num, now_keys, step, min, used
        );*/
        if now_keys == all_keys_num {
            *min = std::cmp::min(*min, step);
            return;
        }
        let x_len = grid.len();
        let y_len = grid[0].len();
        let dirs = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
        for dir in dirs {
            let next_x = dir.0 + x;
            let next_y = dir.1 + y;
            if next_x < 0
                || next_y < 0
                || next_x >= x_len as i32
                || next_y >= y_len as i32
                || used[next_x as usize][next_y as usize]
            {
                continue;
            }
            let next_c = grid[next_x as usize][next_y as usize];
            used[next_x as usize][next_y as usize] = true;
            match next_c {
                '.' => {
                    Self::dfs(
                        grid,
                        (next_x, next_y),
                        keys,
                        all_keys_num,
                        now_keys,
                        step + 1,
                        min,
                        used,
                    );
                }
                'a'..='z' => {
                    keys[next_c as usize - 'a' as usize] = true;
                    Self::dfs(
                        grid,
                        (next_x, next_y),
                        keys,
                        all_keys_num,
                        now_keys + 1,
                        step + 1,
                        min,
                        used,
                    );
                    keys[next_c as usize - 'a' as usize] = false;
                }
                'A'..='Z' => {
                    if keys[next_c as usize - 'A' as usize] {
                        Self::dfs(
                            grid,
                            (next_x, next_y),
                            keys,
                            all_keys_num,
                            now_keys,
                            step + 1,
                            min,
                            used,
                        );
                    }
                }
                _ => {}
            }
            used[next_x as usize][next_y as usize] = false;
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l864() {
        assert_eq!(
            6,
            Solution::shortest_path_all_keys(vec![
                String::from("@..aA"),
                String::from("..B#."),
                String::from("....b")
            ])
        );
        assert_eq!(
            8,
            Solution::shortest_path_all_keys(vec![
                String::from("@.a.#"),
                String::from("###.#"),
                String::from("b.A.B")
            ])
        );

        assert_eq!(
            10,
            Solution::shortest_path_all_keys(vec![
                String::from("@...a"),
                String::from(".###A"),
                String::from("b.BCc")
            ])
        );
    }
}
