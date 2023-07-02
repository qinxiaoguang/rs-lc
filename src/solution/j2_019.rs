use super::Solution;

impl Solution {
    // 最多删除一个字符使其变为回文串
    pub fn valid_palindrome(s: String) -> bool {
        if s.len() == 0 || s.len() == 1 || s.len() == 2 {
            return true;
        }
        let s = s.chars().collect::<Vec<char>>();
        // 两边缩小，遇到不符合的字符串的时候，要么删左边的，要么删右边的
        let (mut left, mut right) = (0, s.len() - 1);

        while left < right {
            if s[left] != s[right] {
                return Self::_is_palindrome(&s[left + 1..=right])
                    || Self::_is_palindrome(&s[left..=right - 1]);
            }
            left += 1;
            right -= 1;
        }
        true
    }
    fn _is_palindrome(s: &[char]) -> bool {
        if s.len() == 0 || s.len() == 1 {
            return true;
        }
        let (mut l, mut r) = (0, s.len() - 1);
        s.iter()
            .enumerate()
            .take(s.len() / 2)
            .find(|(i, c)| **c != s[s.len() - 1 - i])
            .map(|_| false)
            .unwrap_or(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_019() {
        assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
    }
}
