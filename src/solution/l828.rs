use super::Solution;

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        // 统计所有子串的独特字符的个数
        // 因为只有26的字母，所以只需要统计26个字母，每个字母出现的下标
        // 假设A出现在下标 2，5，8的位置，那么包含5这个位置的独立字符串的个数为(5-2)*(8-5)，即5左边可以选0-3个字符，乘上右边可选的8-5个字符

        let s: Vec<char> = s.chars().collect();
        let mut indexs = vec![vec![-1i64; 1]; 26];
        for (i, c) in s.iter().enumerate() {
            indexs[*c as usize - 'A' as usize].push(i as i64);
        }
        let MOD = 1000000007;
        let mut ans = 0i64;
        for i in 0..26 {
            indexs[i].push(s.len() as i64);
            for j in 1..indexs[i].len() - 1 {
                ans += (indexs[i][j] - indexs[i][j - 1]) * (indexs[i][j + 1] - indexs[i][j]);
                ans %= MOD;
            }
        }

        ans as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l828() {
        assert_eq!(Solution::unique_letter_string(String::from("ABC")), 10);
        assert_eq!(Solution::unique_letter_string(String::from("ABA")), 8);
    }
}
