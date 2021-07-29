use super::Solution;

impl Solution {
    // 找规律，1位数有10个，2位数90个，3位数900个
    // 那么他们出现的次数，乘上对应的位个数即可
    // 如3位数需要占用900*3
    // 利用该规律找到该数属于几位数
    // 找到后，除于对应的位数，就知道第几个
    // 继而就知道是几了
    pub fn find_nth_digit(n: i32) -> i32 {
        if n >= 0 && n <= 9 {
            return n;
        }
        // 减去个位数
        let mut n = n as i64;
        let mut digt = 1i64;
        let mut cnt = 1i64;
        while n > 0 {
            n -= 9 * digt * cnt;
            digt *= 10;
            cnt += 1;
        }
        cnt -= 1;
        digt /= 10;
        n += 9 * digt * cnt;
        // 确定该数是cnt位数
        // 确定是第几个数
        let mut count = (n - 1) / cnt;
        let mut yu = n % cnt; // 第几个数的第几位
        if yu == 0 {
            yu = cnt;
        }
        let mut num = count + digt; // 确定最终数
        num /= (10i64.pow(cnt as u32 - yu as u32));
        (num % 10) as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j044() {
        assert_eq!(Solution::find_nth_digit(10), 1);
        assert_eq!(Solution::find_nth_digit(11), 0);
        assert_eq!(Solution::find_nth_digit(12), 1);
        assert_eq!(Solution::find_nth_digit(13), 1);
        assert_eq!(Solution::find_nth_digit(14), 1);
        assert_eq!(Solution::find_nth_digit(15), 2);
        assert_eq!(Solution::find_nth_digit(1000000000), 1);
        assert_eq!(Solution::find_nth_digit(3), 3);
    }
}
