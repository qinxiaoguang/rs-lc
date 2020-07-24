pub struct Solution {}

/*
 * @lc app=leetcode.cn id=414 lang=rust
 *
 * [414] 第三大的数
 *
 * https://leetcode-cn.com/problems/third-maximum-number/description/
 *
 * algorithms
 * Easy (34.50%)
 * Likes:    141
 * Dislikes: 0
 * Total Accepted:    29.5K
 * Total Submissions: 83.6K
 * Testcase Example:  '[3,2,1]'
 *
 * 给定一个非空数组，返回此数组中第三大的数。如果不存在，则返回数组中最大的数。要求算法时间复杂度必须是O(n)。
 *
 * 示例 1:
 *
 *
 * 输入: [3, 2, 1]
 *
 * 输出: 1
 *
 * 解释: 第三大的数是 1.
 *
 *
 * 示例 2:
 *
 *
 * 输入: [1, 2]
 *
 * 输出: 2
 *
 * 解释: 第三大的数不存在, 所以返回最大的数 2 .
 *
 *
 * 示例 3:
 *
 *
 * 输入: [2, 2, 3, 1]
 *
 * 输出: 1
 *
 * 解释: 注意，要求返回第三大的数，是指第三大且唯一出现的数。
 * 存在两个值为2的数，它们都排第二。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        // 此题可以归类到返回一个数组中前k大的数
        // 一般解此题的方案就是，采用小顶堆，将当前堆内的最小值和当前值做比较
        // 如果 比小顶堆的最小值小，则弹出堆顶，并将当前值插入堆中,堆的大小为3，最终堆中三个数，就是top 3
        // 而因为k为3，所以可以不用采用堆，而是直接创建3个临时变量即可。
        let init = std::i64::MIN;

        let (mut small, mut mid, mut large) = (init, init, init);
        for num in nums {
            if num as i64 == small || num as i64 == mid || num as i64 == large {
                continue;
            }
            if num as i64 > small {
                // 更新三者的值，且small弹出
                small = mid;
                mid = large;
                large = num as i64;
                while large < mid || small > large || small > mid {
                    while large < mid {
                        std::mem::swap(&mut large, &mut mid);
                    }
                    while mid < small {
                        std::mem::swap(&mut mid, &mut small);
                    }
                }
            }
            println!("sm:{},m:{},large:{}", small, mid, large);
        }
        if small == init {
            return large as i32;
        }
        return small as i32;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l414() {
        assert_eq!(1, Solution::third_max(vec![3, 2, 1]));
        assert_eq!(2, Solution::third_max(vec![1, 2]));
        assert_eq!(1, Solution::third_max(vec![2, 2, 3, 1]));
        assert_eq!(1, Solution::third_max(vec![1]));
        assert_eq!(1, Solution::third_max(vec![1, 1, 2, 2, 3, 3]));
        assert_eq!(-2147483648, Solution::third_max(vec![1, 2, -2147483648]));
        assert_eq!(0, Solution::third_max(vec![3, 3, 4, 3, 4, 3, 0, 3, 3]));
    }
}
