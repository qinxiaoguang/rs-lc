pub struct Solution {}

/*
 * @lc app=leetcode.cn id=457 lang=rust
 *
 * [457] 环形数组循环
 *
 * https://leetcode-cn.com/problems/circular-array-loop/description/
 *
 * algorithms
 * Medium (33.00%)
 * Likes:    38
 * Dislikes: 0
 * Total Accepted:    4.6K
 * Total Submissions: 13.2K
 * Testcase Example:  '[2,-1,1,2,2]'
 *
 * 给定一个含有正整数和负整数的环形数组 nums。 如果某个索引中的数 k 为正数，则向前移动 k 个索引。相反，如果是负数 (-k)，则向后移动 k
 * 个索引。因为数组是环形的，所以可以假设最后一个元素的下一个元素是第一个元素，而第一个元素的前一个元素是最后一个元素。
 *
 * 确定 nums 中是否存在循环（或周期）。循环必须在相同的索引处开始和结束并且循环长度 >
 * 1。此外，一个循环中的所有运动都必须沿着同一方向进行。换句话说，一个循环中不能同时包括向前的运动和向后的运动。
 *
 *
 * 示例 1：
 *
 * 输入：[2,-1,1,2,2]
 * 输出：true
 * 解释：存在循环，按索引 0 -> 2 -> 3 -> 0 。循环长度为 3 。
 *
 *
 * 示例 2：
 *
 * 输入：[-1,2]
 * 输出：false
 * 解释：按索引 1 -> 1 -> 1 ... 的运动无法构成循环，因为循环的长度为 1 。根据定义，循环的长度必须大于 1 。
 *
 *
 * 示例 3:
 *
 * 输入：[-2,1,-1,-2,-2]
 * 输出：false
 * 解释：按索引 1 -> 2 -> 1 -> ... 的运动无法构成循环，因为按索引 1 -> 2 的运动是向前的运动，而按索引 2 -> 1
 * 的运动是向后的运动。一个循环中的所有运动都必须沿着同一方向进行。
 *
 *
 *
 * 提示：
 *
 *
 * -1000 ≤ nums[i] ≤ 1000
 * nums[i] ≠ 0
 * 0 ≤ nums.length ≤ 5000
 *
 *
 *
 *
 * 进阶：
 *
 * 你能写出时间时间复杂度为 O(n) 和额外空间复杂度为 O(1) 的算法吗？
 *
 */

// @lc code=start
impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        // 条件有如下：1. 环的长度大于1， 2. 环的方向必须向前
        // 不符合条件的话则有：1. 当前点的值为负数 2. 当前点的值为其索引值+1
        // 根据上述情况，从索引0开始依次判断,但是这样得到的结果并不是O(n)
        // 而一个好的方法是，若当前结果并不是环，则将该遍历过程的所有值都修改为0
        let mut nums = nums;

        for idx in 0..nums.len() {
            let dir = nums[idx] > 0;
            if !Self::check(&nums, idx, dir) {
                continue;
            }

            // 符合条件，开始通过快慢指针遍历
            let (mut quick, mut slow) = (idx, idx);
            let len = nums.len();
            loop {
                //println!("quick:{},slow:{}", quick, slow);
                if !Self::check(&nums, quick, dir) || !Self::check(&nums, slow, dir) {
                    // 构不成环
                    break;
                }
                let next_quick = Self::get_next(&nums, quick);
                let next_slow = Self::get_next(&nums, slow);
                if !Self::check(&nums, next_slow as usize, dir) {
                    break;
                }
                let next_slow = Self::get_next(&nums, next_slow);
                if next_quick == next_slow {
                    if Self::check(&nums, next_quick, dir) {
                        return true;
                    } else {
                        break;
                    }
                }
                quick = next_quick;
                slow = next_slow;
            }
            // 构不成环，将所有的路径置-1
            let (mut quick, mut slow) = (idx, idx);
            loop {
                if !Self::check(&nums, quick, dir) {
                    break;
                }
                if !Self::check(&nums, slow, dir) {
                    // 构不成环
                    break;
                }
                let next_quick = Self::get_next(&nums, quick);
                let next_slow = Self::get_next(&nums, slow);
                if !Self::check(&nums, next_slow as usize, dir) {
                    break;
                }
                let next_slow = Self::get_next(&nums, next_slow);
                if next_quick == next_slow {
                    break;
                }
                quick = next_quick;
                slow = next_slow;
                nums[quick] = 0;
                nums[slow] = 0;
            }
        }

        false
    }

    // 检查是否符合条件, dir为true,方向为正，否则为负
    fn check(nums: &Vec<i32>, idx: usize, dir: bool) -> bool {
        //println!("nums:{:?},idx:{}", nums, idx);
        let next_idx = Self::get_next(&nums, idx);
        return !(if dir { nums[idx] <= 0 } else { nums[idx] >= 0 } || next_idx == idx);
    }

    // 获取下一个结点坐标
    fn get_next(nums: &Vec<i32>, now_idx: usize) -> usize {
        let len = nums.len();
        let mut next_idx = now_idx as i32 + nums[now_idx];
        if next_idx <= 0 {
            next_idx += (-next_idx / len as i32 + 1) * len as i32;
        }
        next_idx %= len as i32;
        next_idx as usize
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l457() {
        assert_eq!(true, Solution::circular_array_loop(vec![2, -1, 1, 2, 2]));
        assert_eq!(
            false,
            Solution::circular_array_loop(vec![-2, 1, -1, -2, -2])
        );
        assert_eq!(false, Solution::circular_array_loop(vec![-1, 2]));
        assert_eq!(true, Solution::circular_array_loop(vec![1, 2, 3, 4, 5]));
        assert_eq!(false, Solution::circular_array_loop(vec![1, -2]));
        assert_eq!(true, Solution::circular_array_loop(vec![3, 1, 2]));
        assert_eq!(false, Solution::circular_array_loop(vec![-1, -1, -3]));
    }
}
