pub struct Solution {}
impl Solution {
    // 整数反转,只有一点，就是考虑溢出的情形。
    pub fn reverse(x: i32) -> i32 {
        let mut is_neg = false;
        let mut x: i64 = x as i64;
        if x < 0 {
            is_neg = true;
            x = -x;
        }

        let v = x
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        if (is_neg && v >= -(std::i32::MIN as i64)) || (!is_neg && v >= std::i32::MAX as i64) {
            0
        } else if is_neg {
            -v as i32
        } else {
            v as i32
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l007() {
        assert_eq!(123, Solution::reverse(321));
        assert_eq!(-123, Solution::reverse(-321));
        assert_eq!(21, Solution::reverse(120));
        assert_eq!(0, Solution::reverse(0));
        assert_eq!(0, Solution::reverse(1534236469));
        println!("{}", Solution::reverse(-2147483648))
    }
}
