pub struct Solution {}
/*
 * @lc app=leetcode.cn id=1002 lang=rust
 *
 * [1002] 查找常用字符
 *
 * https://leetcode-cn.com/problems/find-common-characters/description/
 *
 * algorithms
 * Easy (68.52%)
 * Likes:    95
 * Dislikes: 0
 * Total Accepted:    15.6K
 * Total Submissions: 22.8K
 * Testcase Example:  '["bella","label","roller"]'
 *
 * 给定仅有小写字母组成的字符串数组 A，返回列表中的每个字符串中都显示的全部字符（包括重复字符）组成的列表。例如，如果一个字符在每个字符串中出现 3
 * 次，但不是 4 次，则需要在最终答案中包含该字符 3 次。
 *
 * 你可以按任意顺序返回答案。
 *
 *
 *
 * 示例 1：
 *
 * 输入：["bella","label","roller"]
 * 输出：["e","l","l"]
 *
 *
 * 示例 2：
 *
 * 输入：["cool","lock","cook"]
 * 输出：["c","o"]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= A.length <= 100
 * 1 <= A[i].length <= 100
 * A[i][j] 是小写字母
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    // 其实就是求交集
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        // 也可以用vec，毕竟字母都是小写的，顶多26个
        let mut now_map = HashMap::new();
        for c in a[0].chars().collect::<Vec<char>>().into_iter() {
            *now_map.entry(c).or_insert(0) += 1;
        }

        for i in 1..a.len() {
            let mut tmp_map = HashMap::new();
            for c in a[i].chars().collect::<Vec<char>>().into_iter() {
                let cnt = now_map.entry(c).or_insert(0);
                if *cnt > 0 {
                    *cnt -= 1;
                    *tmp_map.entry(c).or_insert(0) += 1;
                }
            }
            now_map = tmp_map;
        }
        let mut res = vec![];
        for (key, value) in now_map {
            for i in 0..value {
                res.push(key.to_string());
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
    fn test_l1002() {
        println!(
            "{:?}",
            Solution::common_chars(vec![
                String::from("bella"),
                String::from("label"),
                String::from("roller")
            ])
        );

        println!(
            "{:?}",
            Solution::common_chars(vec![
                String::from("cool"),
                String::from("lock"),
                String::from("cook")
            ])
        );

        println!("{:?}", Solution::common_chars(vec![String::from("abcda")]));
    }
}
