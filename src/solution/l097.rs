pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 判断s3字符串是否由s1和s2交错而来的
    // 需要使用回溯法，回溯点是，当遍历到的左右两个字符相等时，选择左还是选择右
    // 但是回溯法 会超时，所以要考虑使用dp。
    // 遇到字符串，优先思考使用dp
    // dp[i][j] 表示s1[..i]到s2[..j]可以组成s3[..i+j]
    // 那么dp[i][j+1]就与s2[j+1]和s3[i+j+1]的关系有关了
    // 分四种情况，假设a是s1中当前遍历的字符，b是s2当前遍历的字符，c1c2是s3当前遍历的字符
    // 若 ab=c1c2 或 ba=c1c2 则dp[i][j] = dp[i-1][j-1]
    // 若 a = c2 则dp[i][j] |= dp[i-1][j]
    // 若 b = c2 则dp[i][j] |= dp[i][j-1]
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let s3_bytes = s3.as_bytes();
        dp[0][0] = true;
        for j in 1..=s2.len() {
            dp[0][j] = if s3[..j] == s2[..j] { true } else { false };
        }
        for i in 1..=s1.len() {
            dp[i][0] = if s3[..i] == s1[..i] { true } else { false };
        }
        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                let c1 = s1_bytes[i - 1];
                let c2 = s2_bytes[j - 1];

                let c3_1 = s3_bytes[i + j - 2];
                let c3_2 = s3_bytes[i + j - 1];
                if (c1 == c3_1 && c2 == c3_2) || (c1 == c3_2 && c2 == c3_1) {
                    dp[i][j] = dp[i - 1][j - 1];
                }

                if c1 == c3_2 {
                    dp[i][j] |= dp[i - 1][j];
                }
                if c2 == c3_2 {
                    dp[i][j] |= dp[i][j - 1];
                }
                println!("{},{},{}", i, j, dp[i][j]);
            }
        }
        dp[s1.len()][s2.len()]
    }

    // dfs保留，放这，这个方法会超时
    #[allow(dead_code)]
    fn dfs(s1: &str, s2: &str, s3: &str) -> bool {
        println!("{},{},{}", s1, s2, s3);
        if s1.is_empty() && s2.is_empty() && s3.is_empty() {
            return true;
        }
        if s1.is_empty() {
            return s2.as_bytes() == s3.as_bytes();
        }
        if s2.is_empty() {
            return s1.as_bytes() == s3.as_bytes();
        }
        if s3.is_empty() {
            return false;
        }

        let c1 = s1.as_bytes().first().unwrap();
        let c2 = s2.as_bytes().first().unwrap();
        let c3 = s3.as_bytes().first().unwrap();
        let mut res = false;
        if c1 == c3 {
            res = Self::dfs(&s1[1..], &s2, &s3[1..]);
        }
        if c2 == c3 {
            res |= Self::dfs(&s1, &s2[1..], &s3[1..]);
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l097() {
        assert_eq!(
            true,
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbcbcac".to_string()
            )
        );

        assert_eq!(
            false,
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            )
        );

        assert_eq!(
            true,
            Solution::is_interleave(
                "accbaabaaabbcbaacbababacaababbcbabaababcaabbbbbcacbaa".to_string(),
                "cabaabcbabcbaaaacababccbbccaaabaacbbaaabccacabaaccbbcbcb".to_string(),
                "accbcaaabbaabaaabbcbcbabacbacbababaacaaaaacbabaabbcbccbbabbccaaaaabaabcabbcaabaaabbcbcbbbcacabaaacccbbcbbaacb".to_string()
            )
        );
    }
}
