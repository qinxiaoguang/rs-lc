pub struct Solution {}
impl Solution {
    // 简单正则匹配，.匹配一个字符，*匹配0个或多个前边的一个字符
    // s可能为空，[a-z],p可能为空,[a-z.*]
    // 判断p是否能匹配到s， 如p = .* , 可以匹配s的任意字符
    // 采用dp的思想去解决。dp[i][j] 表示s[0..i]与p[0..j]是否匹配
    // 分情况考虑,首先要知道，当遍历到(i,j)时，(i,j-1),(i-1,j-1),(i-1,j)的答案是已经知道的，所以要利用这3个信息
    // 太难了，写完了，费脑子,又挂了，不想写了。
    pub fn is_match(s: String, p: String) -> bool {
        false
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l010() {
        assert_eq!(false, Solution::is_match("aa".to_string(), "a".to_string()));
        assert_eq!(true, Solution::is_match("aa".to_string(), "a*".to_string()));
        assert_eq!(true, Solution::is_match("ab".to_string(), ".*".to_string()));
        assert_eq!(
            true,
            Solution::is_match("aab".to_string(), "c*a*b*".to_string())
        );
        assert_eq!(
            false,
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string())
        );
        assert_eq!(true, Solution::is_match("".to_string(), ".*".to_string()));
        assert_eq!(false, Solution::is_match("".to_string(), "a".to_string()));
        assert_eq!(false, Solution::is_match("a".to_string(), "".to_string()));
        assert_eq!(true, Solution::is_match("a".to_string(), "a".to_string()));
        assert_eq!(
            true,
            Solution::is_match("aaa".to_string(), "ab*ac*a".to_string())
        );
    }
}
