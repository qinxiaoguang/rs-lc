use super::Solution;

impl Solution {
    // 快速幂
    // 重要的是判断负数的情况，切转成正数是否会溢出等情况
    pub fn my_pow(x: f64, n: i32) -> f64 {
        Self::my_powi64(x, n as i64)
    }
    pub fn my_powi64(x: f64, n: i64) -> f64 {
        if x == 1.0 {
            return x;
        }
        if n < 0 {
            return 1.0 / Self::my_powi64(x, -n);
        }
        let (mut res, mut n, mut num) = (1f64, n, x);
        while n > 0 {
            if n & 1 == 1 {
                res *= num;
            }
            num *= num;
            n >>= 1;
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j016() {
        assert_eq!(Solution::my_pow(2f64, 10), 1024f64);
        println!("Solution::my_pow(2.10,3) = {:?}", Solution::my_pow(2.10, 3));
        println!(
            "Solution::my_pow(2.0, -2) = {:?}",
            Solution::my_pow(2.0, -2)
        );
        println!(
            "Solution::my_pow(2.0, -2147483648) = {:?}",
            Solution::my_pow(2.0, -2147483648)
        );
    }
}
