use super::Solution;

impl Solution {
    // 判断一个字符串是否是回文串
    // 只考虑数字和字母，其他字符忽略，字母也忽略大小写
    pub fn is_palindrome(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let map = |c: char| {
            if c >= '0' && c <= '9' {
                return c;
            }
            if c >= 'a' && c <= 'z' {
                return c;
            }
            if c >= 'A' && c <= 'Z' {
                return (c as u8 - b'A' + b'a') as char;
            }
            '#'
        };

        let s = s.chars().map(map).collect::<Vec<char>>();
        let (mut left, mut right) = (0usize, s.len() - 1);
        while left < right {
            while left < right && s[left] == '#' {
                left += 1;
            }
            if left == right {
                return true;
            }
            while left < right && s[right] == '#' {
                right -= 1;
            }
            if left == right {
                return true;
            }
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_018() {
        assert_eq!(Solution::is_palindrome("".to_string()), true);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
        assert_eq!(Solution::is_palindrome("a".to_string()), true);
        assert_eq!(Solution::is_palindrome("a.".to_string()), true);
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }
}
