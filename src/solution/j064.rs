use super::Solution;

impl Solution {
    pub fn sum_nums(n: i32) -> i32 {
        let mut res = 0;
        Self::sum_nums_ref(n, &mut res);
        res
    }

    fn sum_nums_ref(n: i32, res: &mut i32) -> bool {
        *res += n;
        n == 0 || Self::sum_nums_ref(n - 1, res)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j064() {
        assert_eq!(Solution::sum_nums(1), 1);
        assert_eq!(Solution::sum_nums(2), 3);
        assert_eq!(Solution::sum_nums(3), 6);
    }
}
