pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 升序的旋转数组中是否存在目标值。数组中的数可能重复
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let (mut left, mut right) = (0_usize, nums.len() - 1);
        while left < right {
            let mid = (left + right + 1) / 2;
            println!("left:{},right:{},mid:{}", left, right, mid);
            // 相等的话，无法判断答案落在哪边，所以要去掉该干扰项
            if nums[mid] == nums[left] {
                left += 1;
                continue;
            }
            if nums[mid] == nums[right] {
                right -= 1;
                continue;
            }
            if (nums[mid] > target && nums[left] <= target)
                || (nums[mid] < nums[left] && (target >= nums[left] || target < nums[mid]))
            {
                right = mid - 1
            } else {
                left = mid;
            }
        }
        if left >= nums.len() {
            return false;
        }
        nums[left] == target
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l081() {
        println!("{}", Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
        println!("{}", Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
        println!("{}", Solution::search(vec![2], 3));
        println!("{}", Solution::search(vec![2], 2));
        println!("{}", Solution::search(vec![3, 1], 3));
        println!("{}", Solution::search(vec![1, 3, 1, 1], 3));
    }
}
