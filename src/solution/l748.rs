pub struct Solution {}
/*
 * @lc app=leetcode.cn id=748 lang=rust
 *
 * [748] 最短完整词
 *
 * https://leetcode-cn.com/problems/shortest-completing-word/description/
 *
 * algorithms
 * Easy (58.92%)
 * Likes:    29
 * Dislikes: 0
 * Total Accepted:    7.9K
 * Total Submissions: 13.5K
 * Testcase Example:  '"1s3 PSt"\n["step","steps","stripe","stepple"]'
 *
 *
 * 如果单词列表（words）中的一个单词包含牌照（licensePlate）中所有的字母，那么我们称之为完整词。在所有完整词中，最短的单词我们称之为最短完整词。
 *
 * 单词在匹配牌照中的字母时不区分大小写，比如牌照中的 "P" 依然可以匹配单词中的 "p" 字母。
 *
 * 我们保证一定存在一个最短完整词。当有多个单词都符合最短完整词的匹配条件时取单词列表中最靠前的一个。
 *
 * 牌照中可能包含多个相同的字符，比如说：对于牌照 "PP"，单词 "pair" 无法匹配，但是 "supper" 可以匹配。
 *
 *
 *
 * 示例 1：
 *
 * 输入：licensePlate = "1s3 PSt", words = ["step", "steps", "stripe", "stepple"]
 * 输出："steps"
 * 说明：最短完整词应该包括 "s"、"p"、"s" 以及 "t"。对于 "step" 它只包含一个 "s"
 * 所以它不符合条件。同时在匹配过程中我们忽略牌照中的大小写。
 *
 *
 *
 * 示例 2：
 *
 * 输入：licensePlate = "1s3 456", words = ["looks", "pest", "stew", "show"]
 * 输出："pest"
 * 说明：存在 3 个包含字母 "s" 且有着最短长度的完整词，但我们返回最先出现的完整词。
 *
 *
 *
 *
 * 注意:
 *
 *
 * 牌照（licensePlate）的长度在区域[1, 7]中。
 * 牌照（licensePlate）将会包含数字、空格、或者字母（大写和小写）。
 * 单词列表（words）长度在区间 [10, 1000] 中。
 * 每一个单词 words[i] 都是小写，并且长度在区间 [1, 15] 中。
 *
 *
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut src_map = HashMap::new();
        for c in license_plate.chars() {
            if Self::is_letter(c) {
                *src_map.entry(Self::convert(c)).or_insert(0) += 1
            }
        }
        let mut res = String::from("");
        let mut min_len = std::i32::MAX;
        for word in words {
            let mut tmp = HashMap::new();
            for c in word.chars() {
                *tmp.entry(Self::convert(c)).or_insert(0) += 1;
            }
            let mut exist = true;

            for (k, v) in &src_map {
                if !tmp.contains_key(k) {
                    exist = false;
                    break;
                }
                if tmp.get(k).unwrap() < v {
                    exist = false;
                    break;
                }
            }
            if exist {
                let count = word.chars().count();
                if count < min_len as usize {
                    min_len = count as i32;
                    res = word;
                }
            }
        }

        res
    }

    fn is_letter(c: char) -> bool {
        let v = c as u8;
        let (a, z, A, Z) = ('a' as u8, 'z' as u8, 'A' as u8, 'Z' as u8);
        return (v >= a && v <= z) || (v >= A && v <= Z);
    }

    fn convert(c: char) -> u8 {
        let mut v = c as u8;
        let (a, A, Z) = ('a' as u8, 'A' as u8, 'Z' as u8);
        if v >= A && v <= Z {
            v = v - A + a;
        }
        return v;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l748() {
        assert_eq!(
            String::from("pest"),
            Solution::shortest_completing_word(
                String::from("1s3 456"),
                vec![
                    String::from("looks"),
                    String::from("pest"),
                    String::from("stew"),
                    String::from("show")
                ]
            )
        );
        assert_eq!(
            String::from("steps"),
            Solution::shortest_completing_word(
                String::from("1s3 PSt"),
                vec![
                    String::from("step"),
                    String::from("steps"),
                    String::from("stripe"),
                    String::from("stepple")
                ]
            )
        );
        /*println!("{}", 'a' as u8);
        println!("{}", 'b' as u8);
        println!("{}", 'z' as u8);

        println!("{}", 'A' as u8);
        println!("{}", 'B' as u8);
        println!("{}", 'Z' as u8);*/
    }
}
