pub struct Solution {}
/*
 * @lc app=leetcode.cn id=780 lang=rust
 *
 * [780] 到达终点
 *
 * https://leetcode-cn.com/problems/reaching-points/description/
 *
 * algorithms
 * Hard (27.11%)
 * Likes:    58
 * Dislikes: 0
 * Total Accepted:    1.8K
 * Total Submissions: 6.7K
 * Testcase Example:  '9\n5\n12\n8'
 *
 * 从点 (x, y) 可以转换到 (x, x+y)  或者 (x+y, y)。
 *
 * 给定一个起点 (sx, sy) 和一个终点 (tx, ty)，如果通过一系列的转换可以从起点到达终点，则返回 True ，否则返回 False。
 *
 *
 * 示例:
 * 输入: sx = 1, sy = 1, tx = 3, ty = 5
 * 输出: True
 * 解释:
 * 可以通过以下一系列转换从起点转换到终点：
 * (1, 1) -> (1, 2)
 * (1, 2) -> (3, 2)
 * (3, 2) -> (3, 5)
 *
 * 输入: sx = 1, sy = 1, tx = 2, ty = 2
 * 输出: False
 *
 * 输入: sx = 1, sy = 1, tx = 1, ty = 1
 * 输出: True
 *
 *
 *
 * 注意:
 *
 *
 * sx, sy, tx, ty 是范围在 [1, 10^9] 的整数。
 *
 *
 */

// @lc code=start
impl Solution {
    // 既然是hard, 我觉得使用dfs(2^n)铁定是不行的
    // 当然dfs中也可以使用剪枝来优化，能优化到更低复杂度也有可能是行的。
    // 先使用dfs或bfs尝试一下
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        // 正推不行，可以考虑下倒推，对于tx,ty而言，能否倒推出唯一值呢？
        // 我想答案是一定的，假如 (a,b) 倒推，只能推出 (a, b-a)和(a-b,b)
        // 而因为对于任何一对数组，其内容都是正数，而b-a和a-b只能选一个数出来。当然也有可能a=b
        // 所以对于倒推而言，其是可行的。
        let mut queue = vec![];
        queue.push((tx, ty));
        while queue.len() > 0 {
            let (x, y) = queue.remove(0);
            //println!("x:{},y:{}", x, y);
            if x == sx && y == sy {
                return true;
            }
            if x < sx || y < sy {
                continue;
            }
            if x > y {
                // x-y >y时,下一步依然是x-2y,y,同样的道理
                // 所以应该一步到位，直接减到x-ny小于y为止
                // 但是需要考虑到x-ny的值要比sx大,所以将sx这个因素考虑进去
                let mut next_x = x - (x - sx) / y * y;
                if next_x == x {
                    next_x = x - y;
                }
                queue.push((next_x, y))
            } else if x < y {
                let mut next_y = y - (y - sy) / x * x;
                if next_y == y {
                    next_y = y - x;
                }
                queue.push((x, next_y))
            } else {
                // x == y
                queue.push((x, 0));
                queue.push((0, y));
            }
        }
        false
    }

    // 尝试了，超时，不可行
    pub fn reaching_points_bfs(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let mut queue = vec![];
        queue.push((sx, sy));
        while queue.len() > 0 {
            let (x, y) = queue.remove(0);
            if x == tx && y == ty {
                return true;
            }
            if x > tx || y > ty {
                continue;
            }
            queue.push((x, x + y));
            queue.push((x + y, y));
        }
        false
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l780() {
        assert_eq!(true, Solution::reaching_points(1, 1, 3, 5));
        assert_eq!(false, Solution::reaching_points(1, 1, 2, 2));
        assert_eq!(true, Solution::reaching_points(1, 1, 1, 1));
        assert_eq!(true, Solution::reaching_points(6, 5, 11, 16));
        assert_eq!(true, Solution::reaching_points(1, 1, 1000000000, 1));
        assert_eq!(false, Solution::reaching_points(9, 5, 12, 8));
    }
}
