use super::Solution;

impl Solution {
    // 找出不含有重复字符串的最长连续子字符串的长度
    // 使用hashmap 加滑动窗口
    // 其中map中存储的是字母出现的左下标
    pub fn length_of_longest_substring_v2(s: String) -> i32 {
        let mut m = std::collections::HashMap::<char, i32>::with_capacity(s.len());
        let mut ans = 0;
        let mut left = 0;

        let s: Vec<char> = s.chars().collect();
        for (i, c) in s.iter().enumerate() {
            if m.contains_key(c) {
                while left < i {
                    m.remove(&s[left]);
                    left += 1;
                    if s[left - 1] == *c {
                        break;
                    }
                }
            }
            ans = ans.max(i - left + 1);
            m.insert(*c, i as i32);
            continue;
        }

        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_016() {
        assert_eq!(
            Solution::length_of_longest_substring_v2("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_v2("bbbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring_v2("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring_v2("".to_string()), 0);
    }
}
