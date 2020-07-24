pub struct Solution {}

/*
 * @lc app=leetcode.cn id=605 lang=rust
 *
 * [605] 种花问题
 *
 * https://leetcode-cn.com/problems/can-place-flowers/description/
 *
 * algorithms
 * Easy (31.44%)
 * Likes:    153
 * Dislikes: 0
 * Total Accepted:    29.3K
 * Total Submissions: 91.7K
 * Testcase Example:  '[1,0,0,0,1]\n1'
 *
 * 假设你有一个很长的花坛，一部分地块种植了花，另一部分却没有。可是，花卉不能种植在相邻的地块上，它们会争夺水源，两者都会死去。
 *
 * 给定一个花坛（表示为一个数组包含0和1，其中0表示没种植花，1表示种植了花），和一个数 n 。能否在不打破种植规则的情况下种入 n
 * 朵花？能则返回True，不能则返回False。
 *
 * 示例 1:
 *
 *
 * 输入: flowerbed = [1,0,0,0,1], n = 1
 * 输出: True
 *
 *
 * 示例 2:
 *
 *
 * 输入: flowerbed = [1,0,0,0,1], n = 2
 * 输出: False
 *
 *
 * 注意:
 *
 *
 * 数组内已种好的花不会违反种植规则。
 * 输入的数组长度范围为 [1, 20000]。
 * n 是非负整数，且不会超过输入数组的大小。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        // 计算0的个数的问题
        let mut zero = 0;
        let mut res = 0;
        let mut flowerbed = flowerbed;
        if flowerbed.len() == 0 {
            return false;
        }
        flowerbed.push(0);
        flowerbed.push(1);
        if flowerbed[0] == 0 {
            zero = 1;
        }

        for num in flowerbed {
            if num == 0 {
                zero += 1;
            } else {
                res += (zero - 1) / 2;
                zero = 0;
            }
        }
        res >= n
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l605() {
        assert_eq!(true, Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert_eq!(false, Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }
}
