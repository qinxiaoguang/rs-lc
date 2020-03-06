pub struct Solution {}
impl Solution {
    // 旋转数组找到目标值,二分方法
    // 若左边的数大于右边，则结果可能在此数组中
    // 若左边的数小于右边，则此数组为升序,并按情况判断即可
    // 若存在，返回其索引
    // nums = [4,5,6,7,0,1,2], target = 0,
    // 1,3:3
    #![allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::my_search(&nums, target)
    }

    fn my_search(nums: &[i32], target: i32) -> i32 {
        println!("{:?}", nums);
        if nums.len() == 0 {
            return -1;
        }
        let (start, end) = (0_i32, nums.len() as i32 - 1);
        let mid = (start + end) / 2;
        if nums[mid as usize] == target {
            return mid;
        }
        // 若不等，则根据情况判断
        if mid > start {
            let (left, right) = (nums[start as usize], nums[mid as usize - 1]);
            if (left <= target && target <= right) || left >= right {
                let res = Self::my_search(&nums[start as usize..mid as usize], target);
                if res >= 0 {
                    return res + start;
                }
            }
        }
        if mid < end {
            let (left, right) = (nums[mid as usize + 1], nums[end as usize]);
            if (left <= target && target <= right) || left >= right {
                let res = Self::my_search(&nums[mid as usize + 1..end as usize + 1], target);
                if res >= 0 {
                    return res + mid + 1;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l033() {
        println!("{}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        println!("{}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
        println!("{}", Solution::search(vec![1, 3], 3));
        println!("{}", Solution::search(vec![2, 3, 4, 5, 6, 7, 8, 9, 1], 9));
    }
}
