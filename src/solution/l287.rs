pub struct Solution {}

/*
 * @lc app=leetcode.cn id=287 lang=rust
 *
 * [287] 寻找重复数
 *
 * https://leetcode-cn.com/problems/find-the-duplicate-number/description/
 *
 * algorithms
 * Medium (63.25%)
 * Likes:    776
 * Dislikes: 0
 * Total Accepted:    85.7K
 * Total Submissions: 130.4K
 * Testcase Example:  '[1,3,4,2,2]'
 *
 * 给定一个包含 n + 1 个整数的数组 nums，其数字都在 1 到 n 之间（包括 1 和
 * n），可知至少存在一个重复的整数。假设只有一个重复的整数，找出这个重复的数。
 *
 * 示例 1:
 *
 * 输入: [1,3,4,2,2]
 * 输出: 2
 *
 *
 * 示例 2:
 *
 * 输入: [3,1,3,4,2]
 * 输出: 3
 *
 *
 * 说明：
 *
 *
 * 不能更改原数组（假设数组是只读的）。
 * 只能使用额外的 O(1) 的空间。
 * 时间复杂度小于 O(n^2) 。
 * 数组中只有一个重复的数字，但它可能不止重复出现一次。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // 二分法
        Self::find_duplicatev_mid_find(nums)
        // 还有一个算法是快慢指针，把数组想象成图，那么此图必然存在环，使用快慢指针来找环即可。
    }
    // 二分法
    // 方法就是从1~n中进行二分，二分的判断是小于mid的数的个数是否大于mid，如果是，答案就在左边，如果非答案就在右边，也是一个很巧妙的方法
    fn find_duplicatev_mid_find(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (1, nums.len() - 1);
        while l < r {
            // 向下取整，一般会将l = mid+1
            let mid = (l + r) / 2;

            let cnt = nums
                .iter()
                .fold(0, |cnt, num| if *num <= mid as i32 { cnt + 1 } else { cnt });

            if cnt <= mid {
                l = mid + 1;
            } else {
                r = mid;
            }
            //println!("l:{},r:{}", l, r);
        }
        return l as i32;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l287() {
        assert_eq!(3, Solution::find_duplicate(vec![3, 1, 3, 4, 2]));
        assert_eq!(2, Solution::find_duplicate(vec![1, 3, 4, 2, 2]));
    }
}
