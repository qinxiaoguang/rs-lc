use super::Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        // 每次从数组中选出两个数和等于k的，最简单的方法就是hash表
        use std::collections::HashMap;

        let mut m = HashMap::new();
        nums.iter()
            .map(|&x| *m.entry(x).or_insert(0) += 1)
            .collect::<Vec<_>>();
        let mut res = 0;
        nums.iter()
            .map(|&x| {
                if let Some(0) = m.get(&x) {
                    return ();
                }
                *m.entry(x).or_insert(0) -= 1;
                let cnt = m.entry(k - x).or_insert(0);
                if *cnt <= 0 {
                    *m.entry(x).or_insert(0) += 1;
                    return ();
                }
                *cnt -= 1;
                res += 1;
            })
            .collect::<Vec<_>>();

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1679() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
        assert_eq!(Solution::max_operations(vec![], 6), 0);
        assert_eq!(Solution::max_operations(vec![1], 6), 0);
    }
}
