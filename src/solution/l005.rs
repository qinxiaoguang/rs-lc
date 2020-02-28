pub struct Solution {}
impl Solution {
    // 最长回文子串
    // babad => aba
    pub fn longest_palindrome(s: String) -> String {
        // 首先需要将s变为#b#a#b#a#d
        let mut res = s.chars().map(|c| format!("#{}", c)).collect::<String>();
        res = format!("{}#", res);
        let chars: Vec<char> = res.chars().collect();
        let mut rl: Vec<i32> = vec![1; chars.len()];
        let (mut pos, mut right_max) = (0_i32, 0_i32);
        let mut max_len = 0_i32;
        let (mut start, mut end) = (0, 0);

        for i in 1..chars.len() {
            if i < right_max as usize {
                let j = 2 * pos - i as i32;
                if j > 0 {
                    // 注意需要比较大小，因为有可能，对称的位置的值很大，再对称过来超过了right_max的值
                    rl[i] = if rl[j as usize] < right_max - i as i32 {
                        rl[j as usize]
                    } else {
                        right_max - i as i32
                    }
                }
            }
            // 从i点的i-rl[i] 及i+rl[i]点开始遍历，查找最长串
            loop {
                let left_i = i as i32 - rl[i];
                let right_i = i as i32 + rl[i];
                if left_i < 0 || right_i >= chars.len() as i32 {
                    break;
                }
                if chars[left_i as usize] == chars[right_i as usize] {
                    rl[i] += 1;
                } else {
                    break;
                }
            }
            max_len = if rl[i] > max_len {
                start = (i as i32 - rl[i] + 1) as usize;
                end = (i as i32 + rl[i] - 1) as usize;
                rl[i]
            } else {
                max_len
            };
            let right_max_tmp = rl[i] + i as i32 - 1;
            right_max = if right_max_tmp > right_max {
                pos = i as i32;
                right_max_tmp
            } else {
                right_max
            };
        }
        chars
            .iter()
            .enumerate()
            .filter(|(i, c)| i >= &start && i <= &end)
            .map(|(i, c)| c)
            .filter(|&c| c != &'#')
            .collect::<String>()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l000() {
        assert_eq!("h", Solution::longest_palindrome("heihei".to_string()));
        assert_eq!("bab", Solution::longest_palindrome("babad".to_string()));
        assert_eq!("bb", Solution::longest_palindrome("cbbd".to_string()));
        assert_eq!("", Solution::longest_palindrome("".to_string()));
        assert_eq!(
            "adada",
            Solution::longest_palindrome("babadada".to_string())
        );
    }
}
