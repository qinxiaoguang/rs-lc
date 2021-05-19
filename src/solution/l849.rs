use super::Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        // 就是找到两个人之间的最大距离
        // 但是要考虑边界情况，第一个人离第一个作为的距离，以及最后一个人离最后一个位置的距离
        let start = seats
            .iter()
            .enumerate()
            .find(|x| *x.1 == 1)
            .map(|x| x.0)
            .unwrap();
        let mut max = start as i32;
        let end = seats
            .iter()
            .enumerate()
            .rfind(|x| *x.1 == 1)
            .map(|x| x.0)
            .unwrap();
        max = std::cmp::max(max, (seats.len() - 1 - end) as i32);

        // 从start 到end找间距
        let mut tmp_max = 0;
        for i in start + 1..=end {
            if seats[i] == 0 {
                tmp_max += 1;
            } else {
                max = std::cmp::max(max, (tmp_max + 1) / 2);
                tmp_max = 0;
            }
        }
        max
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_849() {
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0]), 3);
        assert_eq!(Solution::max_dist_to_closest(vec![0, 1]), 1);
    }
}
