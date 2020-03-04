pub struct Solution {}
macro_rules! sf {
    ($str:expr) => {
        String::from($str)
    };
}
impl Solution {
    // 给定字符串，找出里面的最长的有效括号的字符串的长度
    // 1. 采用栈辅助，遇到(将当前坐标入栈，遇到)弹出栈顶，并用当前坐标计算其差值，即入栈的只有(的坐标
    // 2. 使用dp,dp[i]表示以i结尾的最长有效括号长度，则当s[i]为 "("时，dp[i]=0,
    //    当s[i]为")"时，则s[i-1]为"("时，则dp[i] = 2+dp[i-2]
    //                    s[i-1]为")"时，则s[i-dp[i-1]]为(时，则dp[i] = 2+dp[i-1]
    // 其余情况dp[i]为0
    // 3. 最巧妙的解法，就是使用尺取法，但是使用两次，一次是对正序的s进行尺取，一次再对倒序的s进行尺取。
    pub fn longest_valid_parentheses(s: String) -> i32 {
        std::cmp::max(
            Self::cal_long(&s),
            Self::cal_long(
                &(s.chars()
                    .rev()
                    .map(|c| match c {
                        '(' => ')',
                        _ => '(',
                    })
                    .collect::<String>()),
            ),
        )
    }

    pub fn cal_long(s: &str) -> i32 {
        let (mut left_num, mut right_num) = (0_i32, 0_i32);
        let (mut left, mut right) = (0_usize, 0_usize);
        let mut max = 0_i32;
        let chars: Vec<char> = s.chars().collect();
        //println!("chars:{:?}", chars);
        while left < chars.len() && right < chars.len() {
            match chars[right] {
                '(' => {
                    left_num += 1;
                }
                _ => {
                    right_num += 1;
                }
            };
            right += 1;
            if left_num == right_num {
                max = std::cmp::max(max, right as i32 - left as i32);
                //println!("{},{},{},{}", left, right, left_num, right_num);
                continue;
            }
            if left_num < right_num {
                left = right;
                left_num = 0;
                right_num = 0;
            }
            //println!("{},{},{},{}", left, right, left_num, right_num);
        }
        max
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l032() {
        assert_eq!(
            4,
            Solution::longest_valid_parentheses(String::from(")()())"))
        );
        assert_eq!(2, Solution::longest_valid_parentheses(String::from("(()")));
    }
}
