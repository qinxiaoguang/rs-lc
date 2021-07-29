use super::Solution;

impl Solution {
    // 不能用加减乘除 计算两数之和
    // 采用位运算异或，其别名为非进位加法
    // 所以每个位最终的结果，可以通过两次异或来计算，并通过&来计算是否进位
    pub fn add(a: i32, b: i32) -> i32 {
        // 负数需要处理
        let (mut a, mut b) = (a, b);
        let mut res = 0;
        let mut front = 0; // 是否进位
        let mut cnt = 0;
        while a != 0 || b != 0 {
            let (la, lb) = (a & 1, b & 1);
            let tmp = la ^ lb ^ front;
            front = if la & lb == 1 || la & front == 1 || lb & front == 1 {
                1
            } else {
                0
            };
            res = res ^ (tmp << cnt);
            a >>= 1;
            b >>= 1;
            cnt += 1;
        }
        res = res ^ (front << cnt);
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j065() {
        assert_eq!(Solution::add(1, 1), 2);
        assert_eq!(Solution::add(1, 2), 3);
        for l in 0..100 {
            for j in 0..100 {
                assert_eq!(Solution::add(l, j), l + j);
            }
        }
    }
}
