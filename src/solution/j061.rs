use super::Solution;
use std::process::exit;

impl Solution {
    pub fn is_straight(nums: Vec<i32>) -> bool {
        if nums.len() != 5 {
            return false;
        }
        // 计算最左侧的true和最右侧的true中间有几个不是true
        let mut zero_cnt = 0;
        let mut set = vec![false; 14];
        let mut min_idx = 14usize;
        nums.iter().for_each(|i| {
            set[*i as usize] = true;
            if *i == 0 {
                zero_cnt += 1;
            } else {
                min_idx = min_idx.min(*i as usize);
            }
        });
        let mut res = true;
        if min_idx + 5 >= 14 {
            min_idx = 9;
        }
        (min_idx..min_idx + 5).find(|x| {
            if !set[*x] {
                zero_cnt -= 1;
                if zero_cnt < 0 {
                    res = false;
                    return true;
                }
            }
            return false;
        });
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j061() {
        assert_eq!(Solution::is_straight(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(Solution::is_straight(vec![0, 0, 1, 2, 3]), true);
        assert_eq!(Solution::is_straight(vec![0, 0, 2, 2, 5]), false);
        assert_eq!(Solution::is_straight(vec![0, 12, 11, 11, 0]), false);
    }
}
