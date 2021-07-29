use super::Solution;

impl Solution {
    // dp, 类似蚂蚁爬楼梯
    // 因为一个字母可以有两个数翻译成，也可以由1个数字翻译成
    // 所以设置dp[i]表示以i为底的，有多少翻译方法
    // 那么dp[i]的更新规则为dp[i] = dp[i-1] + if dp[i-2..i] between 10..26 {dp[i-2]}
    pub fn translate_num(num: i32) -> i32 {
        let cs: Vec<char> = num.to_string().chars().collect();
        let mut dp = vec![0; cs.len() + 1];
        dp[0] = 1;
        dp[1] = 1; //
        for i in 1..cs.len() {
            dp[i + 1] = dp[i];
            let n = cs[i - 1..=i]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if n >= 10 && n <= 25 {
                dp[i + 1] += dp[i - 1];
            }
        }

        dp[cs.len()]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j046() {
        assert_eq!(Solution::translate_num(12258), 5);
        assert_eq!(Solution::translate_num(26), 1);
        assert_eq!(Solution::translate_num(506), 1);
    }
}
