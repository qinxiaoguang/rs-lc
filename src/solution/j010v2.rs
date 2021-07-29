use super::Solution;

impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        let mut n = n + 1;
        if n == 0 || n == 1 || n == 2 {
            return (n + 1) / 2;
        }
        let (mut f, mut s) = (1, 1);
        let mut n = n - 2;
        let MOD = 1000000007;
        while n > 0 {
            let tmp = s;
            s = (f + s) % MOD;
            f = tmp;
            n -= 1;
        }
        s
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_010() {
        assert_eq!(Solution::num_ways(0), 1);
        assert_eq!(Solution::num_ways(1), 1);
        assert_eq!(Solution::num_ways(2), 2);
        assert_eq!(Solution::num_ways(7), 21);
    }
}
