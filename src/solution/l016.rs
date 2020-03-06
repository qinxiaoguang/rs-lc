pub struct Solution {}
impl Solution {
    // 找到数组中，三数相加为0的数
    // 必须采用固定第一个数，查找后边的数中的目标值，因为两个数相加等于某数，可以采用o(n)的解法
    // 和15题讨论一样
    #![allow(dead_code)]
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut min = std::i32::MAX;
        let mut res = -1_i32;
        nums.sort();
        if nums.len() <= 2 {
            return 0;
        }
        for i in 0..nums.len() - 2 {
            //println!("i:{},nums[i]:{}", i, nums[i]);
            let (min_dis, min_sum) = Self::two_sum_closest(&nums[i + 1..], target - nums[i]);
            //println!("min_dis:{},min_sum:{}", min_dis, min_sum);
            if min_dis == 0 {
                return min_sum + nums[i];
            }
            if min >= min_dis {
                min = min_dis;
                res = min_sum + nums[i];
            }
        }
        res
    }

    fn two_sum_closest(nums: &[i32], target: i32) -> (i32, i32) {
        //println!("nums:{:?},target:{}", nums, target);
        let mut min_dis = std::i32::MAX;
        let mut min_sum = 0_i32;
        let (mut left, mut right) = (0_i32, nums.len() as i32 - 1);
        while left < right {
            let sum = nums[left as usize] + nums[right as usize];
            let abs_dis = (sum - target).abs();
            if target - sum >= 0 {
                left += 1;
            } else {
                right -= 1;
            }

            if abs_dis < min_dis {
                min_dis = abs_dis;
                min_sum = sum;
            }
        }
        (min_dis, min_sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l016() {
        println!("{:?}", Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
        //        println!("{:?}", Solution::three_sum_closest(vec![], 1));
    }
}
