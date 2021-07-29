use super::Solution;

impl Solution {
    // 所有奇数位于数组前半部分，偶数位于后半部分
    // 双指针可解
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return nums;
        }
        let mut nums = nums;
        let (mut l, mut r) = (0usize, nums.len() - 1);

        while l < r {
            // l找偶数
            while l < r && nums[l] & 1 != 0 {
                l += 1;
            }
            // r找奇数
            while l < r && nums[r] & 1 == 0 {
                r -= 1;
            }
            // 交换l r
            nums.swap(l, r);
        }
        nums
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j021() {
        println!(
            "Solution::exchange(vec![1,2,3,4]) = {:?}",
            Solution::exchange(vec![1, 2, 3, 4])
        );
    }
}
