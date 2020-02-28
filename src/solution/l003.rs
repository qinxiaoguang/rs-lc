pub struct Solution {}
impl Solution {
    // abcabcbb => 3, 非重复子串的最长长度
    // 通过滑动窗口的思路去解决，要点是，
    // 1. 保证在遍历start~end时，保存在set中的字符是不重复的。
    // 2. 当出现重复的字符时，需要再遍历一遍当前的start~end的序列，删除对应的字符，直到遇到重复的字符，同时更新start的位置。
    // 3. 边界条件
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        if len == 0 {
            return 0;
        }
        let mut max = 1_i32;
        let mut start = 0;
        let mut set = HashSet::new();
        for end in 0..len {
            if set.contains(&seq[end]) {
                let tmp = (end - start) as i32;
                max = if tmp > max { tmp } else { max };
                // 从start位置依次往左遍历
                for i in start..end {
                    set.remove(&seq[i]);
                    if &seq[i] == &seq[end] {
                        start = i + 1;
                        break;
                    }
                }
            }
            // no contains
            set.insert(&seq[end]);
        }
        let tmp = (len - start) as i32;
        if tmp > max {
            tmp
        } else {
            max
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l003() {
        assert_eq!(
            3_i32,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        );
        assert_eq!(
            1_i32,
            Solution::length_of_longest_substring("bbbb".to_string())
        );
        assert_eq!(
            3_i32,
            Solution::length_of_longest_substring("pwwkew".to_string())
        );
        assert_eq!(0_i32, Solution::length_of_longest_substring("".to_string()));
        assert_eq!(
            3_i32,
            Solution::length_of_longest_substring("dvdf".to_string())
        );
        assert_eq!(
            2_i32,
            Solution::length_of_longest_substring("au".to_string())
        );
    }
}
