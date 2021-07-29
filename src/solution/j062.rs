use super::Solution;

impl Solution {
    // 竟然管这题叫简单题 = =
    // 假设f(n,m)表示最后剩下的第几个元素
    // 那么就可以递归来解,首先删除第一个元素，剩下n-1个元素要删除，那么只需要求出f(n-1,m)即可，在当前的数组基础上，找到该数即可
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        Self::j062_f(n, m)
    }

    // 计算n,m会留下第几个
    fn j062_f(n: i32, m: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        // 可以改成迭代的方式
        (m + Self::j062_f(n - 1, m)) % n
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j062() {
        assert_eq!(Solution::last_remaining(5, 3), 3);
        assert_eq!(Solution::last_remaining(10, 17), 2);
    }
}
