use super::Solution;

impl Solution {
    // 计算买卖一次股票的最大利润，因为一次，所以不用dp
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut l, mut r) = (vec![0; prices.len()], vec![0; prices.len()]);
        // 先从左往右遍历，更新l
        let mut min = i32::MAX;
        prices.iter().enumerate().for_each(|(x, &val)| {
            min = std::cmp::min(min, val);
            l[x] = min;
        });
        let mut max = i32::MIN;
        prices.iter().enumerate().rev().for_each(|(x, &val)| {
            max = std::cmp::max(max, val);
            r[x] = max;
        });
        let mut res = 0;
        (0..prices.len()).for_each(|i| res = std::cmp::max(r[i as usize] - l[i as usize], res));
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j063() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
}
