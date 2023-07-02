use std::ops::Index;

use super::Solution;

impl Solution {
    // 找两个完全没有相同字符的长度乘积最大值，因为字符串是a-z,所以可以将字符串转换为2进制
    pub fn max_product(words: Vec<String>) -> i32 {
        let bit_words: Vec<(i32, usize)> = words
            .iter()
            .map(|str| {
                let mut convert_res = 0;
                str.chars().for_each(|char| {
                    convert_res |= 1 << char as i32 - 'a' as i32;
                });
                (convert_res, str.len())
            })
            .collect();

        let mut ans = 0i32;
        for i in 0..bit_words.len() {
            for j in i + 1..bit_words.len() {
                if bit_words[i].0 & bit_words[j].0 == 0 {
                    let tmp_res = (bit_words[i].1 * bit_words[j].1) as i32;
                    if ans < tmp_res {
                        ans = tmp_res;
                    }
                }
            }
        }
        return ans;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_005() {
        assert_eq!(
            Solution::max_product(vec![
                "abcw".to_string(),
                "baz".to_string(),
                "foo".to_string(),
                "bar".to_string(),
                "fxyz".to_string(),
                "abcdef".to_string()
            ]),
            16
        );
        assert_eq!(
            Solution::max_product(vec![
                "a".to_string(),
                "ab".to_string(),
                "abc".to_string(),
                "d".to_string(),
                "cd".to_string(),
                "bcd".to_string(),
                "abcd".to_string()
            ]),
            4
        );
    }
}
