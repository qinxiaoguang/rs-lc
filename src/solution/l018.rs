pub struct Solution {}
impl Solution {
    // 找到数组中，三数相加为0的数
    // 必须采用固定第一个数，查找后边的数中的目标值，因为两个数相加等于某数，可以采用o(n)的解法
    // 即(i,j)两个指针，如果当前和大于目标值，则j--,否则i++
    #![allow(dead_code)]
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res = std::collections::HashSet::new();
        if nums.len() <= 3 {
            return vec![];
        }
        nums.sort();
        for i in 0..nums.len() - 3 {
            for j in i + 1..nums.len() - 2 {
                let tmp = nums[i] + nums[j];
                let two_res = Self::two_sum(&nums[j + 1..], target - tmp);
                two_res.into_iter().for_each(|v| {
                    if v.len() >= 2 {
                        res.insert(vec![nums[i], nums[j], v[0], v[1]]);
                    }
                });
            }
        }
        res.into_iter().collect::<Vec<Vec<i32>>>()
    }
    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let (mut left, mut right) = (0_i32, nums.len() as i32 - 1);
        while left < right {
            let sum = nums[left as usize] + nums[right as usize];
            if sum > target {
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                res.push(vec![nums[left as usize], nums[right as usize]]);
                left += 1;
                right -= 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l018() {
        println!("{:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0));
        println!("{:?}", Solution::four_sum(vec![-3, -1, 0, 2, 4, 5], 1));
    }
}
