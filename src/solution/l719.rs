pub struct Solution {}
/*
 * @lc app=leetcode.cn id=719 lang=rust
 *
 * [719] 找出第 k 小的距离对
 *
 * https://leetcode-cn.com/problems/find-k-th-smallest-pair-distance/description/
 *
 * algorithms
 * Hard (33.80%)
 * Likes:    108
 * Dislikes: 0
 * Total Accepted:    5K
 * Total Submissions: 14.8K
 * Testcase Example:  '[1,3,1]\n1'
 *
 * 给定一个整数数组，返回所有数对之间的第 k 个最小距离。一对 (A, B) 的距离被定义为 A 和 B 之间的绝对差值。
 *
 * 示例 1:
 *
 *
 * 输入：
 * nums = [1,3,1]
 * k = 1
 * 输出：0
 * 解释：
 * 所有数对如下：
 * (1,3) -> 2
 * (1,1) -> 0
 * (3,1) -> 2
 * 因此第 1 个最小距离的数对是 (1,1)，它们之间的距离为 0。
 *
 *
 * 提示:
 *
 *
 * 2 <= len(nums) <= 10000.
 * 0 <= nums[i] < 1000000.
 * 1 <= k <= len(nums) * (len(nums) - 1) / 2.
 *
 *
 */

// @lc code=start
impl Solution {
    // 该题是hard难度，明显不能用暴力解决，暴力方法其实挺好解决，o(n^2)计算出所有数对，并排序得出第k小的即可
    // O(n^2)既然不行，那基本上要考虑O(nlogn)了，首先想到的当然是对数对排序
    // 接着类似赛马，每个赛道每次找出一个最小值，首先每个赛道的最小值是相邻的两个数差值
    // 接着将每个赛道的最小值放入优先队列中，在优先队列中取最小值，同时将该赛道的次小值放入
    // 假如该赛道的最小值为(i,j),那么次小值则为(i,j+1),循环上次过程，直到取到结果，
    // 但是当k很大的时候，导致时间复杂度变得比O(n^2)还大，所以这种方法不行。

    // 看了官方题解，其采用的二分查找法，刚开始还纳闷，这tm怎么用二分，看了才知道，他的二分是针对结果二分的
    // 因为结果肯定在[0, Max(nums)-Min(nums)]中，所以每次都猜一个数(二分的数)，若数对中小于guess的个数小于k,那么结果在[guess..]中
    // 关键是怎么求小于guess的数对的个数，这个是关键。没看懂，先放弃
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut pairs = vec![];
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                pairs.push((nums[i] - nums[j]).abs());
            }
        }
        pairs.sort();
        pairs[k as usize - 1]
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l719() {
        assert_eq!(0, Solution::smallest_distance_pair(vec![1, 3, 1], 1));
    }
}
