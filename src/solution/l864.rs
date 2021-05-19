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
    // 如果不限制步数，那么他可能回在两个点来回走。
    // 看了官方题解，采用bfs来计算起点，钥匙点等各个关键点的距离
    // 之后通过dijstra来求解，只能说一句牛比
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let grid: Vec<Vec<char>> = grid
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        let mut cost = vec![vec![0; 7]; 7];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    'a'..='k' => {
                        Self::get_length(&grid, (i, j), &mut cost);
                    }
                    _ => {}
                }
            }
        }
        0
    }

    // 获取i,j点到各个关键点的距离
    // 0为起点，1~6分别对应a~f,若没有某个字母，则该字母对应的所有的距离都是0
    fn get_length(grid: &Vec<Vec<char>>, (i, j): (usize, usize), cost: &mut Vec<Vec<i32>>) {}
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
