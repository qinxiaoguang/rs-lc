pub struct Solution {}
impl Solution {
    // 实现 pow(x, n) ，即计算 x 的 n 次幂函数
    // 即 使用快速幂来计算
    #[allow(dead_code)]
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut bottom = x;
        let mut res = 1_f64;
        let mut n = n as i64;
        let is_neg = if n >= 0 {
            false
        } else {
            n = -n;
            true
        };
        while n > 0 {
            if n % 2 == 1 {
                // n-=1 也可以没有，因为n>>=1 会自动把奇数中的n-=1消除掉
                n -= 1;
                res *= bottom;
            }
            n >>= 1;
            bottom *= bottom;
        }
        if is_neg {
            return 1.0_f64 / res;
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l050() {
        println!("{}", Solution::my_pow(2.0_f64, 10));
        println!("{}", Solution::my_pow(2.1_f64, 3));
        println!("{}", Solution::my_pow(2.0_f64, -2));
        println!("{}", Solution::my_pow(2.0_f64, 0));
        println!("{}", Solution::my_pow(2.0_f64, std::i32::MIN));
    }
}
