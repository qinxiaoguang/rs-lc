pub struct Solution {}
impl Solution {
    // 计算雨水能容纳的最大面积
    // 维护start,end两个下标，分别从最后和最前来计算当前能容纳的最大面积
    // 找出两个下标中的最小值，让最小值进行遍历，重复上述步骤。
    // 为什么是最小值遍历呢，因为决定面积的是最小值。
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0_i32;
        }
        let (mut start, mut end) = (0, height.len() - 1);
        let mut max_area = 0_i32;
        while start <= end {
            let (start_num, end_num) = (height[start], height[end]);
            let min_num = std::cmp::min(start_num, end_num);
            let now_area = min_num * (end as i32 - start as i32);
            max_area = std::cmp::max(max_area, now_area);
            if start_num > end_num {
                end -= 1;
                continue;
            }
            start += 1;
        }
        max_area
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l011() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
        assert_eq!(0, Solution::max_area(vec![]));
        assert_eq!(0, Solution::max_area(vec![1]));
    }
}
