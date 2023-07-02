use super::Solution;

impl Solution {
    // 返回所有的三数相加为0的
    // 此题是2数相加的变版，但需要注意的是去重
    // 去重只需要固定第一位，从后边的数组中找两数之和
    // 第一位在往后移动的时候，记得去重就可以了
    // 而两数之和的去重，则在遍历的时候去重就可以了
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        // 先排序
        let mut nums = nums.clone();
        nums.sort();
        let mut last = nums[0];
        let mut res = vec![];
        for i in 0..nums.len() {
            if i != 0 && last == nums[i] {
                continue;
            }
            // 查找结果中两数相加和为-nums[i]的
            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let sum = nums[left] + nums[right];
                if sum == -nums[i] {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    while left < right && nums[left - 1] == nums[left] {
                        left += 1;
                    }
                    right -= 1;
                    while left < right && nums[right + 1] == nums[right] {
                        right -= 1;
                    }
                    continue;
                }
                if sum < -nums[i] {
                    left += 1;
                    continue;
                }
                right -= 1;
            }

            last = nums[i];
        }

        return res;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_007() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![[-1, -1, 2], [-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), vec![vec![]; 0]);
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![[0, 0, 0]]);
        assert_eq!(Solution::three_sum(vec![]), vec![vec![]; 0]);
        assert_eq!(Solution::three_sum(vec![1, 1]), vec![vec![]; 0]);
    }
}
