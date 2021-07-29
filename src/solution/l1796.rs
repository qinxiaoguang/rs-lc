use super::Solution;

impl Solution {
    // 字符串中第2大的数,保存两个变量即可
    pub fn second_highest(s: String) -> i32 {
        let (mut big, mut small) = (-1, -2);
        s.chars().for_each(|c| {
            let v = c as i32 - '0' as i32;
            if v >= 0 && v < 10 {
                if v < small || v == big || v == small {
                    return ();
                }
                if v > big {
                    // 大于big
                    small = big;
                    big = v;
                } else {
                    // 小于big但大于small的
                    small = v;
                }
            }
        });
        if small < 0 {
            small = -1;
        }
        small
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1796() {
        assert_eq!(Solution::second_highest(s!("dfa12321afd")), 2);
        assert_eq!(Solution::second_highest(s!("abc1111")), -1);
    }
}
