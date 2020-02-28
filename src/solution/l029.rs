pub struct Solution {}
impl Solution {
    // 不使用除法，乘法，mod计算两个数相除的商
    // 不使用除法，但是可以使用加法，减法，此题可以考虑快速蜜
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut is_neg = false;
        if (dividend > 0 && divisor < 0) || (dividend < 0 && divisor > 0) {
            is_neg = true
        }
        let (mut cnt, _) = Self::my_divide((dividend as i64).abs(), (divisor as i64).abs());
        if is_neg {
            cnt = -cnt;
        }
        if is_neg && cnt <= std::i32::MIN as i64 {
            std::i32::MIN
        } else if !is_neg && cnt >= std::i32::MAX as i64 {
            std::i32::MAX
        } else {
            cnt as i32
        }
    }

    // 类似快速蜜的方式，divisor在下次递归的时候会变为两倍
    // 第二个数是mod后的值
    fn my_divide(dividend: i64, divisor: i64) -> (i64, i64) {
        if dividend < divisor {
            return (0, dividend);
        }
        let mut dividend = dividend;
        if divisor <= dividend && dividend < divisor + divisor {
            let (mut cnt, mut mod_v) = Self::my_divide(dividend - divisor, divisor);
            cnt += 1;
            return (cnt, mod_v);
        } else {
            let mut res = 0;
            while dividend >= divisor + divisor {
                let (cnt, mod_v) = Self::my_divide(dividend, divisor + divisor);
                res += cnt + cnt;
                dividend = mod_v;
            }
            if dividend < divisor {
                return (res, dividend);
            } else {
                let (cnt, mod_v) = Self::my_divide(dividend, divisor);
                return (res + cnt, mod_v);
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l029() {
        assert_eq!(2147483647, Solution::divide(-2147483648, -1));
        assert_eq!(3, Solution::divide(10, 3));
        assert_eq!(-2, Solution::divide(7, -3));
    }
}
