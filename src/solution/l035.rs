pub struct Solution {}

impl Solution {
    // 找目标值，若不存在，找到应该插入的位置
    // 其实就是找到第一个大于等于该数的位置
    // 一定要明白，二分法的模式就是下面那一套模板
    // 需要注意的要点是，当数组只有1，2，3个数时，二分的表现是什么,因为这是临界条件
    // 主要的要点是，怎么更新left和right的值,以及二分的位置是向上取整还是向下取整，
    // 一般来说，向下取整就是 (left+right)/2, 而向上取整就是 (left+right+1)/2
    #![allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0_usize, nums.len() - 1);
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if nums[left] < target {
            left += 1;
        }
        left as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l035() {
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
        assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
    }
}
