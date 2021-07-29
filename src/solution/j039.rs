use super::Solution;

impl Solution {
    // 方法有很多，比如排序，map等
    // 但最主要的方法是投票法
    // 如果发现后边的值和前边的值一样，那么该票数就加1，若票数变为0 ，则最高票为下一个公民
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut res, mut count) = (nums[0], 1);
        for i in 1..nums.len() {
            if count == 0 {
                count = 1;
                res = nums[i];
                continue;
            }
            if nums[i] == res {
                count += 1;
            } else {
                count -= 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j039() {
        println!(
            "Solution::majority_element(vec! [1, 2, 3, 2, 2, 2, 5, 4, 2]) = {:?}",
            Solution::majority_element(vec![1, 2, 3, 2, 2, 2, 5, 4, 2])
        );

        println!(
            "Solution::majority_element(vec![1,2,2]) = {:?}",
            Solution::majority_element(vec![1, 2, 2])
        );
    }
}
