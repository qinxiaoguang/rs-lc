pub struct Solution {}
impl Solution {
    // 找数组中的字符串中的最长公共前缀
    #![allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res: Vec<char> = vec![];
        let min_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);
        let chars_vec: Vec<Vec<char>> = strs
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        for i in 0..min_len {
            let same_char = chars_vec[0][i];
            for j in 1..chars_vec.len() {
                if same_char != chars_vec[j][i] {
                    return res.iter().collect::<String>();
                }
            }
            res.push(same_char);
        }
        res.iter().collect::<String>()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l014() {
        assert_eq!(
            "fl",
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
        assert_eq!(
            "",
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );
        assert_eq!(
            "",
            Solution::longest_common_prefix(vec![
                "".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );
        assert_eq!("", Solution::longest_common_prefix(vec![]));
        assert_eq!("a", Solution::longest_common_prefix(vec!["a".to_string()]));
    }
}
