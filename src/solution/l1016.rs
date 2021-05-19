pub struct Solution {}
/*
 * @lc app=leetcode.cn id=1016 lang=rust
 *
 * [1016] 子串能表示从 1 到 N 数字的二进制串
 *
 * https://leetcode-cn.com/problems/binary-string-with-substrings-representing-1-to-n/description/
 *
 * algorithms
 * Medium (57.62%)
 * Likes:    24
 * Dislikes: 0
 * Total Accepted:    3.6K
 * Total Submissions: 6.3K
 * Testcase Example:  '"0110"\n3'
 *
 * 给定一个二进制字符串 S（一个仅由若干 '0' 和 '1' 构成的字符串）和一个正整数 N，如果对于从 1 到 N 的每个整数 X，其二进制表示都是 S
 * 的子串，就返回 true，否则返回 false。
 *
 *
 *
 * 示例 1：
 *
 * 输入：S = "0110", N = 3
 * 输出：true
 *
 *
 * 示例 2：
 *
 * 输入：S = "0110", N = 4
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= S.length <= 1000
 * 1 <= N <= 10^9
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    // 1. 暴力法，按照1~32的滑动窗口把所有的子串的值都取出来做记录
    // 这样遍历的时间复杂度是 (2n-32)/16 ,即O(n)的复杂度
    // 肯定有更好的方法，目前想不出来
    // 2. 擦嘞，将1~n转换成字符串，使用s.contains()来判断
    // 这个好像更简单 = =
    // 我这猪脑子
    pub fn query_string(s: String, n: i32) -> bool {
        let cs: Vec<char> = s.chars().collect();
        let mut exist = HashMap::new();
        for len in 1..32 {
            let (mut start, mut end) = (0, len);
            while end <= cs.len() {
                let num =
                    i32::from_str_radix(&cs[start..end].iter().collect::<String>(), 2).unwrap();
                //println!("start:{},end:{},num:{}", start, end, num);
                exist.entry(num).or_insert(true);
                end += 1;
                start += 1;
            }
        }
        for i in 1..=n {
            if !exist.contains_key(&i) {
                return false;
            }
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1016() {
        assert_eq!(true, Solution::query_string(String::from("0110"), 3));
        assert_eq!(false, Solution::query_string(String::from("0110"), 4));
    }
}
