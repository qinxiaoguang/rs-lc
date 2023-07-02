use super::Solution;

impl Solution {
    // 字符串转换成整数
    pub fn str_to_int(str: String) -> i32 {
        const START:i32 = 0;
        const SPACE:i32 = 1;
        const CHAR:i32 = 2;
        const NUM:i32 = 3;
        const NUM_SPACE:i32 = 4;
        let mut state = START;
        let mut neg = 0;
        let mut res:i64 = 0;
        let mut flow = 0;
        let mut cal = |t:char| {
            res = res*10 + (t as i64-'0' as i64);
            if res > i32::MAX as i64 {
                flow = 1;
                res = 0;
            }
        };
        for c in str.chars() {
            match state {
                START|SPACE =>  {
                    state = match c {
                        ' ' => SPACE,
                        '-' => {
                            neg = 1;
                            CHAR
                        },
                        '+' => CHAR,
                        t@'0'..='9'=> {
                            cal(t);
                            NUM 
                        },
                        _ => return 0,
                    };
                },
                CHAR => {
                    state = match c {
                        t @ '0'..='9' => {
                            cal(t);
                            NUM
                        }
                        _ => return 0,
                    };
                }
                NUM => {
                    state = match c {
                        t @ '0'..='9' => {
                            cal(t);
                            NUM
                        },
                        _ => NUM_SPACE,
                    };
                }
                _ => {}
            }
        };

        if flow == 1{
            return if neg == 1 {i32::MIN} else {i32::MAX};
        };
        if neg == 1 {res = -res};
        if res >= i32::MAX as i64 {i32::MAX} else if res <= i32::MIN as i64 {i32::MIN} else {res as i32}
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j067() {
        assert_eq!(Solution::str_to_int("42".to_string()),42);
        assert_eq!(Solution::str_to_int("-42".to_string()),-42);
        assert_eq!(Solution::str_to_int("4194 with words".to_string()),4194);
        assert_eq!(Solution::str_to_int("words and 987".to_string()),0);
        assert_eq!(Solution::str_to_int("9223372036854775808".to_string()),2147483647);
    }
}
