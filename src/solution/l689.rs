pub struct Solution {}
/*
 * @lc app=leetcode.cn id=689 lang=rust
 *
 * [689] 三个无重叠子数组的最大和
 *
 * https://leetcode-cn.com/problems/maximum-sum-of-3-non-overlapping-subarrays/description/
 *
 * algorithms
 * Hard (46.07%)
 * Likes:    74
 * Dislikes: 0
 * Total Accepted:    1.7K
 * Total Submissions: 3.7K
 * Testcase Example:  '[1,2,1,2,6,7,5,1]\n2'
 *
 * 给定数组 nums 由正整数组成，找到三个互不重叠的子数组的最大和。
 *
 * 每个子数组的长度为k，我们要使这3*k个项的和最大化。
 *
 * 返回每个区间起始索引的列表（索引从 0 开始）。如果有多个结果，返回字典序最小的一个。
 *
 * 示例:
 *
 *
 * 输入: [1,2,1,2,6,7,5,1], 2
 * 输出: [0, 3, 5]
 * 解释: 子数组 [1, 2], [2, 6], [7, 5] 对应的起始索引为 [0, 3, 5]。
 * 我们也可以取 [2, 1], 但是结果 [1, 3, 5] 在字典序上更大。
 *
 *
 * 注意:
 *
 *
 * nums.length的范围在[1, 20000]之间。
 * nums[i]的范围在[1, 65535]之间。
 * k的范围在[1, floor(nums.length / 3)]之间。
 *
 *
 */

// @lc code=start
impl Solution {
    // 好题啊好题，就看你能想到不能
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // w表示以index为底的长度为k个数的窗口的和
        let mut w = vec![];
        let mut sum: i32 = nums[..k as usize].iter().sum();
        w.push(sum);
        for i in 1..nums.len() - k as usize + 1 {
            sum -= nums[i - 1];
            sum += nums[i as usize + k as usize - 1];
            w.push(sum);
        }
        //w
        // left[i]表示w[0,i]中的出现的最大值的下标
        let mut left = vec![0; w.len()];
        let mut best = 0;
        for i in 0..w.len() {
            if w[i] > w[best as usize] {
                best = i;
            }
            left[i] = best;
        }

        // right[i]表示w[i..]中出现的最大值的下标
        let mut right = vec![0; w.len()];
        best = w.len() - 1;
        for i in (0..w.len()).rev() {
            if w[i] >= w[best as usize] {
                best = i;
            }
            right[i] = best;
        }
        // 假设固定了中间数j,那么左侧结果i肯定在w的[0..j-k]中(因为不能重合)
        // 同样的右侧结果z则在w的[j+k..]中，这样只需要遍历一遍就可以知道结果了
        let mut ans = vec![-1i32, -1, -1];
        for j in k..w.len() as i32 - k {
            let i = left[j as usize - k as usize];
            let z = right[j as usize + k as usize];
            if ans[0] == -1
                || w[i] + w[j as usize] + w[z]
                    > w[ans[0] as usize] + w[ans[1] as usize] + w[ans[2] as usize]
            {
                ans[0] = i as i32;
                ans[1] = j as i32;
                ans[2] = z as i32;
            }
        }

        ans
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l689() {
        assert_eq!(
            vec![0, 3, 5],
            Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2)
        );
    }
}
