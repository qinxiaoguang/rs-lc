use super::Solution;

impl Solution {
    // dp题，关键是状态怎么转移,今天不做
    // .表示任意字符，*表示其前边一个字符可以出现0~多次
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        // 更新dp[0][j]
        for j in 0..p.len() {
            if p[j] == '*' {
                dp[0][j + 1] = dp[0][j - 1];
            }
        }
        for i in 0..s.len() {
            for j in 0..p.len() {
                match p[j] {
                    '.' => {
                        dp[i + 1][j + 1] = dp[i][j];
                    }
                    '*' => {
                        // TODO 处理p[0]是*的情况
                        if j as i32 - 2 >= 0 && p[j - 1] != '*' {
                            // 因为出现*的表示前边这个字符可以被取0次
                            dp[i + 1][j + 1] = dp[i + 1][j - 1];
                        }
                        if !dp[i + 1][j + 1] {
                            if s[i] == p[j - 1] || p[j - 1] == '.' {
                                dp[i + 1][j + 1] = dp[i][j] || dp[i + 1][j];
                            }
                            if p[j - 1] == '.' {
                                dp[i + 1][j + 1] = dp[i + 1][j + 1] || dp[i][j + 1];
                            }
                        }
                    }
                    _ => {
                        // 字母
                        if s[i] == p[j] {
                            dp[i + 1][j + 1] = dp[i][j];
                        }
                    }
                }
            }
        }
        dp[s.len()][p.len()]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j019() {
        assert_eq!(Solution::is_match(s!("aa"), s!("a*")), true);
        assert_eq!(Solution::is_match(s!("aa"), s!("a")), false);
        assert_eq!(Solution::is_match(s!("ab"), s!(".*")), true);
        assert_eq!(Solution::is_match(s!("aab"), s!("c*a*b")), true);
        assert_eq!(
            Solution::is_match(s!("mississippi"), s!("mis*is*p*.")),
            false
        );

        assert_eq!(Solution::is_match(s!("aaa"), s!(".*")), true);
    }
}
