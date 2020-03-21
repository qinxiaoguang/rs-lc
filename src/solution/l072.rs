pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 计算Word1到word2的最小编辑距离
    // 可以对单词进行 替换/删除/添加一个字符的操作 ，一次操作的步长为1
    // horse => ros
    // 利用dp, dp[i][j] 表示word[0..i] => word2[0..j]的最小步数
    // 那么就有以下三种情况
    // 1. dp[i][j] = dp[i-1][j-1] (word1[i] == word[j])
    // 2. dp[i][j] = dp[i-1][j] + 1 (增加)
    // 3. dp[i][j] = dp[i][j-1] + 1 (删除)
    // 3. dp[i][j] = dp[i-][j-1] + 1 (替换)
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1.len() == 0 {
            return word2.len() as i32;
        }
        if word2.len() == 0 {
            return word1.len() as i32;
        }
        let mut dp = vec![vec![std::i32::MAX; word2.len() + 1]; word1.len() + 1];
        for i in 0..word1.len() {
            dp[i][0] = i as i32;
        }
        for j in 0..word2.len() {
            dp[0][j] = j as i32;
        }
        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1]
                } else {
                    dp[i][j] =
                        std::cmp::min(std::cmp::min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1])
                            + 1;
                }
            }
        }
        println!("{:?}", dp);
        dp[word1.len()][word2.len()]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l072() {
        assert_eq!(
            3,
            Solution::min_distance("horse".to_string(), "ros".to_string())
        );

        assert_eq!(1, Solution::min_distance("a".to_string(), "b".to_string()));
    }
}
