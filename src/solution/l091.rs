pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // a-z 都通过[1~26]的数字表示，那么给一个数字，求转换为字母有几种方式
    // 如12可以转为ab及l(第12个字母)
    // 采用dp的思想，dp[n] = if s[n] != 0 {dp[n-1]} + if s[n-1..] <= 26 {dp[n-2]}
    pub fn num_decodings(s: String) -> i32 {
        let len = s.len();
        if len == 0 {
            return 0;
        }
        if len == 1 {
            if s.parse::<i32>().unwrap_or(0) == 0 {
                return 0;
            }
            return 1;
        }
        let mut dp = vec![0; len + 1];
        let bytes = s.as_bytes();
        dp[0] = 1;
        dp[1] = if bytes[0] - b'0' != 0 { 1 } else { 0 }; // dp[1]表示第一个数字有几种排法,那么结果就是dp[len]
        for i in 2..=len {
            let last = bytes[i - 2] - b'0';
            let now = bytes[i - 1] - b'0';
            let num = last * 10 + now;
            dp[i] = if now != 0 { dp[i - 1] } else { 0 }
                + if num <= 26 && num >= 10 { dp[i - 2] } else { 0 };
        }
        dp[len]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l091() {
        assert_eq!(2, Solution::num_decodings("12".to_string()));
        assert_eq!(3, Solution::num_decodings("226".to_string()));
        assert_eq!(1, Solution::num_decodings("10".to_string()));
        assert_eq!(0, Solution::num_decodings("00".to_string()));
        assert_eq!(0, Solution::num_decodings("01".to_string()));
        assert_eq!(0, Solution::num_decodings("001".to_string()));
    }
}
