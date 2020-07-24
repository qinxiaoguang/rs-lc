pub struct Solution {}

/*
 * @lc app=leetcode.cn id=229 lang=rust
 *
 * [229] 求众数 II
 *
 * https://leetcode-cn.com/problems/majority-element-ii/description/
 *
 * algorithms
 * Medium (42.84%)
 * Likes:    218
 * Dislikes: 0
 * Total Accepted:    17.9K
 * Total Submissions: 41.4K
 * Testcase Example:  '[3,2,3]'
 *
 * 给定一个大小为 n 的数组，找出其中所有出现超过 ⌊ n/3 ⌋ 次的元素。
 *
 * 说明: 要求算法的时间复杂度为 O(n)，空间复杂度为 O(1)。
 *
 * 示例 1:
 *
 * 输入: [3,2,3]
 * 输出: [3]
 *
 * 示例 2:
 *
 * 输入: [1,1,1,3,3,2,2,2]
 * 输出: [1,2]
 *
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        // O(n)时间复杂度，则只能遍历X*n次，X是固定值
        //  空间O(1)表明不能用map
        //  此题比较经典，采用摩尔投票法,摩尔算法怎么容易理解呢
        // 就好比红警， 每个阵营都有很多小人，找小人数大于1/3的阵营
        // 最多只有2个阵营符合条件，所以每次3个小兵进行抵消，最后剩下的小兵一定是小兵最多的那个阵营

        let (mut count1, mut count2) = (0, 0);
        let (mut value1, mut value2) = (std::i32::MAX, std::i32::MAX);
        for num in &nums {
            if *num == value1 {
                count1 += 1;
            } else if *num == value2 {
                count2 += 1;
            } else {
                if count1 == 0 {
                    value1 = *num;
                    count1 = 1;
                } else if count2 == 0 {
                    value2 = *num;
                    count2 = 1;
                } else {
                    count1 -= 1;
                    count2 -= 1;
                }
            }
            /*println!(
                "value1:{},cnt1:{},value2:{},count2:{}",
                value1, count1, value2, count2
            );*/
        }
        let mut res = vec![];

        let bound = nums.len() / 3;
        /*println!(
            "value1:{},value2:{},count1:{},count2:{}",
            value1, value2, count1, count2
        );*/
        let (cnt1, cnt2) = nums.iter().fold((0, 0), |(mut cnt1, mut cnt2), &num| {
            if num == value1 {
                cnt1 += 1;
            } else if num == value2 {
                cnt2 += 1
            };
            (cnt1, cnt2)
        });
        //println!("cnt1:{},cnt2:{}", cnt1, cnt2);
        if cnt1 > bound {
            res.push(value1);
        }
        if cnt2 > bound {
            res.push(value2);
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l229() {
        assert_eq!(
            vec![1, 2],
            Solution::majority_element(vec![1, 2, 2, 3, 2, 1, 1, 3])
        );
        assert_eq!(vec![3], Solution::majority_element(vec![3, 2, 3]));
        assert_eq!(
            vec![1, 2],
            Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2])
        );
    }
}
