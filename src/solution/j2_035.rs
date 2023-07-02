use super::Solution;
use super::*;
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    // HH:MM, 找出列表中的最小时间差
    // 转换为分钟后按时间排序,并计算每两个之间的顺序以及最后一个和第一个差值
    // 如果结果大于等于12*60的，则使用24减去对应值
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points: Vec<i32> = time_points
            .iter()
            .map(|t| t[..2].parse::<i32>().unwrap() * 60 + t[3..].parse::<i32>().unwrap())
            .collect();

        time_points.sort();

        let mut ans = time_points[0] + 24 * 60 - time_points[time_points.len() - 1];
        for i in 0..time_points.len() - 1 {
            ans = ans.min(time_points[i + 1] - time_points[i]);
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_035() {
        assert_eq!(
            Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()]),
            1
        );
        assert_eq!(
            Solution::find_min_difference(vec![
                "00:00".to_string(),
                "23:59".to_string(),
                "00:00".to_string()
            ]),
            0
        );
    }
}
