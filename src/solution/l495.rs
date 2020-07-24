pub struct Solution {}
/*
 * @lc app=leetcode.cn id=495 lang=rust
 *
 * [495] 提莫攻击
 *
 * https://leetcode-cn.com/problems/teemo-attacking/description/
 *
 * algorithms
 * Medium (52.93%)
 * Likes:    91
 * Dislikes: 0
 * Total Accepted:    10.9K
 * Total Submissions: 20.3K
 * Testcase Example:  '[1,4]\n2'
 *
 * 在《英雄联盟》的世界中，有一个叫 “提莫”
 * 的英雄，他的攻击可以让敌方英雄艾希（编者注：寒冰射手）进入中毒状态。现在，给出提莫对艾希的攻击时间序列和提莫攻击的中毒持续时间，你需要输出艾希的中毒状态总时长。
 *
 * 你可以认为提莫在给定的时间点进行攻击，并立即使艾希处于中毒状态。
 *
 *
 *
 * 示例1:
 *
 * 输入: [1,4], 2
 * 输出: 4
 * 原因: 第 1 秒初，提莫开始对艾希进行攻击并使其立即中毒。中毒状态会维持 2 秒钟，直到第 2 秒末结束。
 * 第 4 秒初，提莫再次攻击艾希，使得艾希获得另外 2 秒中毒时间。
 * 所以最终输出 4 秒。
 *
 *
 * 示例2:
 *
 * 输入: [1,2], 2
 * 输出: 3
 * 原因: 第 1 秒初，提莫开始对艾希进行攻击并使其立即中毒。中毒状态会维持 2 秒钟，直到第 2 秒末结束。
 * 但是第 2 秒初，提莫再次攻击了已经处于中毒状态的艾希。
 * 由于中毒状态不可叠加，提莫在第 2 秒初的这次攻击会在第 3 秒末结束。
 * 所以最终输出 3 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 你可以假定时间序列数组的总长度不超过 10000。
 * 你可以假定提莫攻击时间序列中的数字和提莫攻击的中毒持续时间都是非负整数，并且不超过 10,000,000。
 *
 *
 */

// @lc code=start
impl Solution {
    // 该题类似 求数组的叠加区间 的简化版，可以用那种方式来解题
    // 但有更简单的方法，就是迭代，找到最大值
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut time_series = time_series;
        if time_series.len() == 0 {
            return 0;
        }
        time_series.sort();
        let mut res = duration;
        let mut max_time = time_series[0] + duration;
        for i in 1..time_series.len() {
            if time_series[i] > max_time {
                res += duration;
                max_time = time_series[i] + duration;
                continue;
            }
            if time_series[i] + duration > max_time {
                let add = time_series[i] + duration - max_time;
                res += add;
                max_time = time_series[i] + duration;
            }
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l495() {
        assert_eq!(4, Solution::find_poisoned_duration(vec![1, 4], 2));
        assert_eq!(3, Solution::find_poisoned_duration(vec![1, 2], 2));
    }
}
