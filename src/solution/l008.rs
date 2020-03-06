pub struct Solution {}
impl Solution {
    // 整数转换
    // 1. 去掉前边的空格
    // 2. 若第一个有效字符不是数字或者负号，则返回0
    // 3. 找到前几个有效数字并转换。
    #![allow(dead_code)]
    pub fn my_atoi(str: String) -> i32 {
        let mut prefix_trim = true;
        let mut is_neg = false;
        let mut always_false = false;
        let s = str
            .chars()
            .filter(|&c| {
                if always_false {
                    return false;
                }
                if prefix_trim && c == ' ' {
                    return false;
                }
                if c != ' ' {
                    if c == '-' && prefix_trim {
                        is_neg = true;
                        prefix_trim = false;
                        return true;
                    }
                    if c == '+' && prefix_trim {
                        is_neg = false;
                        prefix_trim = false;
                        return true;
                    }
                    prefix_trim = false;
                    if (c as i32 >= '0' as i32) && (c as i32 <= '9' as i32) {
                        return true;
                    }
                }
                always_false = true;
                return false;
            })
            .collect::<String>();
        //println!("{}", s);
        let v: i64;
        if s.len() == 0 {
            return 0;
        }
        if s.len() == 1 && (s == "+" || s == "-") {
            return 0;
        }
        if is_neg {
            v = s.parse::<i64>().unwrap_or(std::i32::MIN as i64);
        } else {
            v = s.parse::<i64>().unwrap_or(std::i32::MAX as i64);
        }
        if v <= (std::i32::MIN) as i64 {
            std::i32::MIN
        } else if v >= (std::i32::MAX) as i64 {
            std::i32::MAX
        } else {
            v as i32
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l008() {
        assert_eq!(42, Solution::my_atoi("42".to_string()));
        assert_eq!(-42, Solution::my_atoi("-42".to_string()));
        assert_eq!(-42, Solution::my_atoi("  -42".to_string()));
        assert_eq!(-42, Solution::my_atoi("  -42 a 7".to_string()));
        assert_eq!(4193, Solution::my_atoi("4193 with a".to_string()));
        assert_eq!(0, Solution::my_atoi("words and 987".to_string()));
        assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_string()));
        assert_eq!(1, Solution::my_atoi("+1".to_string()));
        assert_eq!(0, Solution::my_atoi("++1".to_string()));
        assert_eq!(1, Solution::my_atoi("1+1".to_string()));
        assert_eq!(
            2147483647,
            Solution::my_atoi("20000000000000000000".to_string())
        );
    }
}
