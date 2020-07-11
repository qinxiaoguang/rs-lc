pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 求某数的平方根，可以用二分法
    // 但此题的思想，应该是想让你用牛顿迭代法
    // 牛顿迭代法就是先随机选一个数，然后计算该点的导数方程，找到导数方程在y=0处的解，下次选择的点即为该处的点,并进行收敛
    // 这里就用二分法来计算吧
    pub fn my_sqrt(x: i32) -> i32 {
        // 一个数的的一半+1一般绝对不是此数的算术平方根
        let (mut left, mut right) = (0, x / 2 + 1);
        while left < right {
            let mid: i64 = (left as i64 + right as i64 + 1) / 2;
            if mid * mid > x as i64 {
                right = mid as i32 - 1;
            } else {
                left = mid as i32;
            }
        }
        left
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l069() {
        println!("{}", Solution::my_sqrt(4));
        println!("{}", Solution::my_sqrt(8));
        println!("{}", Solution::my_sqrt(9));
    }
}
