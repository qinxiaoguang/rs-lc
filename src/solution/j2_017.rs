use super::Solution;
use std::collections::HashMap;

impl Solution {
    // 计算s中包含t所有字符的最短子字符串，多个结果返回一个，没有的话返回空字符串
    // 我们把字符中没在t中出现的字符都当做*号，那么只需要关注出现在t中的字符
    // 使用滑动窗口的方式来解，right一直遍历，直到找到符合条件的窗口
    // 此时left可以尽量缩小， 直到缩小到不符合条件的窗口,再以上述方式依次便利
    pub fn min_window(s: String, t: String) -> String {
        let mut window_map = HashMap::new();
        let mut target_map = HashMap::new();
        let (mut left, mut right) = (0usize, 0usize);

        t.chars()
            .for_each(|c| *target_map.entry(c).or_insert(0) += 1);

        // 检测是否符合条件
        let check = |window_map: &HashMap<char, i32>, target_map: &HashMap<char, i32>| {
            if window_map.len() < target_map.len() {
                return false;
            }
            for (k, v) in target_map.iter() {
                if let Some(s) = window_map.get(k) {
                    if s < v {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            return true;
        };

        let s: Vec<char> = s.chars().collect();
        let mut ans_len = usize::MAX;
        let mut ans = String::from("");

        while left <= right && right < s.len() {
            *window_map.entry(s[right]).or_insert(0) += 1;
            let mut check_res = check(&window_map, &target_map);
            // 符合条件
            while check_res && left <= right {
                if ans_len > right - left {
                    ans_len = right - left;
                    ans = s[left..=right].iter().collect();
                }
                *window_map.entry(s[left]).or_insert(0) -= 1;
                left += 1;
                check_res = check(&window_map, &target_map);
            }
            // 不符合条件
            right += 1;
            continue;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_017() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
        assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
    }
}
