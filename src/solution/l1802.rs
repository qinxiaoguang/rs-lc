use super::Solution;

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        // 模拟, 第一层所有都是1,然后从符合条件的区间(l,r)从两边往中间填
        let mut res = 1;
        let mut max_sum = max_sum - n;
        let (mut l, mut r) = (index, index + 1);
        let mut interval = r - l;
        while max_sum >= interval {
            if interval == n {
                res += max_sum / interval;
                max_sum %= interval;
            } else {
                res += 1;
                max_sum -= interval;
                l = if l < 1 { l } else { l - 1 };
                r = if r >= n { n } else { r + 1 };
            }
            interval = (r - l);
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1802() {
        assert_eq!(Solution::max_value(6, 1, 10), 3);
        assert_eq!(Solution::max_value(4, 2, 6), 2);
        assert_eq!(Solution::max_value(1, 0, 109), 109);
        assert_eq!(Solution::max_value(2, 0, 109), 55);
        assert_eq!(Solution::max_value(8, 7, 14), 4);
        assert_eq!(Solution::max_value(2, 0, 798870804), 399435402);
    }
}
