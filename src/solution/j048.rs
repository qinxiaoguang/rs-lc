use super::Solution;

impl Solution {
    // 滑动窗口，加map
    // 但注意的是在找到最大值的时候，只需要滑动l+r即可，因为最大值已经找到
    // 所以l减小时，r也要同步增大
    pub fn length_of_longest_substring(s: String) -> i32 {
        let input: Vec<i32> = s.chars().map(|c| c as i32 - 'a' as i32).collect();
        if input.len() == 0 {
            return 0;
        }
        let mut cnt = std::collections::HashMap::new();
        let (mut l, mut r) = (0, 0);
        let mut res = 0i32;
        let mut repeat = 0;
        println!("input = {:?}", input);
        while r < input.len() {
            if *cnt.get(&input[r]).unwrap_or(&0) > 0 {
                res = res.max((r - l) as i32);
                // 滑动到下一个相同窗口数，且不重复的地方
                repeat = 0;
                while r < input.len() {
                    let input_l_cnt = cnt.entry(input[l]).or_insert(0);
                    *input_l_cnt -= 1;
                    if *input_l_cnt == 1 {
                        repeat -= 1;
                    }
                    let input_r_cnt = cnt.entry(input[r]).or_insert(0);
                    *input_r_cnt += 1;
                    if *input_r_cnt == 2 {
                        repeat += 1;
                    }
                    l += 1;
                    r += 1;
                    if repeat == 0 {
                        break;
                    }
                }
            } else {
                // cnt[input[r]] == 0
                let input_r_cnt = cnt.entry(input[r]).or_insert(0);
                *input_r_cnt += 1;
                r += 1;
                // 需要处理结尾
                if r == input.len() {
                    return res.max((r - l) as i32);
                }
            }
        }
        res
    }

    // 标准答案
    // 这里的山路十八弯
    pub fn length_of_longest_substring_answer(s: String) -> i32 {
        use std::cmp::max;
        let mut last: [i32; 128] = [-1; 128];
        let mut left = -1;
        let mut ans = 0;
        for (i, v) in s.chars().enumerate() {
            left = max(left, last[v as usize]);
            last[v as usize] = i as i32;
            ans = max(ans, (i as i32) - left);
        }
        return ans;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j048() {
        assert_eq!(Solution::length_of_longest_substring(s!("abcabcbb")), 3);
        assert_eq!(Solution::length_of_longest_substring(s!("bbbbb")), 1);
        assert_eq!(Solution::length_of_longest_substring(s!("pwwkew")), 3);
        assert_eq!(Solution::length_of_longest_substring(s!("a")), 1);
        assert_eq!(Solution::length_of_longest_substring(s!("abbabbababb")), 2);
        assert_eq!(Solution::length_of_longest_substring(s!(" ")), 1);
    }
}
