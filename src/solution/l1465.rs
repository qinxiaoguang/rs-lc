use super::Solution;

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        // 需要计算两个数组中间隔的最大值，提前将0和h放入数组中进行计算，但前提需要排序
        let m: i32 = 10i32.pow(9) + 7;
        let mut horizontal_cuts = horizontal_cuts;
        let mut vertical_cuts = vertical_cuts;
        let (h_max, w_max) = (
            Self::max_interval(&mut horizontal_cuts, h),
            Self::max_interval(&mut vertical_cuts, w),
        );

        ((h_max as i64 * w_max as i64) % m as i64) as i32
    }

    fn max_interval(v: &mut [i32], top: i32) -> i32 {
        use std::cmp::max;
        if v.len() == 0 {
            return 0;
        }
        v.sort();
        let mut max_interval = v[0];
        for i in 1..v.len() {
            let tmp = v[i] - v[i - 1];
            max_interval = max(max_interval, tmp);
        }

        max(max_interval, top - v.last().unwrap())
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1465() {
        assert_eq!(Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
        assert_eq!(Solution::max_area(5, 4, vec![3, 1], vec![1]), 6);
        assert_eq!(Solution::max_area(5, 4, vec![3], vec![3]), 9);
    }
}
