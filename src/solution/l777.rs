pub struct Solution {}
/*
 * @lc app=leetcode.cn id=777 lang=rust
 *
 * [777] 在LR字符串中交换相邻字符
 *
 * https://leetcode-cn.com/problems/swap-adjacent-in-lr-string/description/
 *
 * algorithms
 * Medium (31.97%)
 * Likes:    62
 * Dislikes: 0
 * Total Accepted:    3.6K
 * Total Submissions: 11.2K
 * Testcase Example:  '"X"\n"L"'
 *
 * 在一个由 'L' , 'R' 和 'X'
 * 三个字符组成的字符串（例如"RXXLRXRXL"）中进行移动操作。一次移动操作指用一个"LX"替换一个"XL"，或者用一个"XR"替换一个"RX"。现给定起始字符串start和结束字符串end，请编写代码，当且仅当存在一系列移动操作使得start可以转换成end时，
 * 返回True。
 *
 *
 *
 * 示例 :
 *
 * 输入: start = "RXXLRXRXL", end = "XRLXXRRLX"
 * 输出: True
 * 解释:
 * 我们可以通过以下几步将start转换成end:
 * RXXLRXRXL ->
 * XRXLRXRXL ->
 * XRLXRXRXL ->
 * XRLXXRRXL ->
 * XRLXXRRLX
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= len(start) = len(end) <= 10000。
 * start和end中的字符串仅限于'L', 'R'和'X'。
 *
 *
 */

// @lc code=start
impl Solution {
    // 想了半天没想到好方法
    // 官方题解有个思路很强，就是将L和R想象成面向Left和面向R的实体人，而X为当前位置没有人。
    // 那么此题意思就是这些人行走能否达到目标位置。
    // 注意LR是无法变成RL的，原因就是两个人不能相互穿插通过。
    // 那么如果start和end满足两个特性，就认为是可以转换的
    // 1. 除去X后，两个字符串相等。(LR无法穿插)
    // 2. start中第n个L都要比end中的第n个L的下标要大，第n个R要比end中的第n个R要小
    pub fn can_transform(start: String, end: String) -> bool {
        if start.len() != end.len() {
            return false;
        }
        if start.replace("X", "") != end.replace("X", "") {
            return false;
        }

        let (mut lstart, mut lend, mut rstart, mut rend) = (vec![], vec![], vec![], vec![]);
        for (idx, c) in start.chars().enumerate() {
            if c == 'L' {
                lstart.push(idx);
            } else if c == 'R' {
                rstart.push(idx);
            }
        }
        for (idx, c) in end.chars().enumerate() {
            if c == 'L' {
                lend.push(idx);
            } else if c == 'R' {
                rend.push(idx);
            }
        }

        lstart
            .into_iter()
            .zip(lend.into_iter())
            .all(|(a, b)| a >= b)
            && rstart
                .into_iter()
                .zip(rend.into_iter())
                .all(|(a, b)| a <= b)
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l777() {
        assert_eq!(
            true,
            Solution::can_transform(String::from("RXXLRXRXL"), String::from("XRLXXRRLX"))
        );
    }
}
