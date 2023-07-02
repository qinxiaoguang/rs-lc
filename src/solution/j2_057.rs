use super::*;
use std::collections::BTreeSet;
use std::ops::Bound::Included;
impl Solution {
    // 给你一个整数数组 nums 和两个整数 k 和 t 。请你判断是否存在 两个不同下标 i 和 j，使得 abs(nums[i] - nums[j]) <= t ，同时又满足 abs(i - j) <= k 。
    // 如果存在则返回 true，不存在返回 false。
    // 有序集合的滑动窗口,遍历x时，只需要判断是否元素大于x-t且小于x+t的。
    // rust的有序集合一般用BtreeSet
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut w: BTreeSet<i64> = BTreeSet::new(); // 窗口长度为t

        for (i, v) in nums.iter().enumerate() {
            // 在滑动窗口中查找是否有 [x-k,x+k]的值
            if w.range((
                Included(*v as i64 - t as i64),
                Included(*v as i64 + t as i64),
            ))
            .count()
                > 0
            {
                return true;
            }
            w.insert(*v as i64);
            while w.len() > k as usize {
                w.remove(&(nums[i + 1 - w.len()] as i64));
            }
        }

        return false;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_057() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2),
            true
        );

        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
            false
        );
    }
}
