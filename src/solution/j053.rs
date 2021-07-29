use super::Solution;

impl Solution {
    // 统计在排序数组中出现的次数
    // 也即找目标值极左的坐标和极右的坐标
    // 先查找极右，再从0..极右的坐标中查找极左
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let right = Self::search_target(&nums, false, target);
        if right == -1 {
            return 0;
        }
        let left = Self::search_target(&nums[0..right as usize], true, target);
        if left == -1 {
            return 1;
        }
        right - left + 1
    }

    // 返回对应的下标, 如果不存在返回-1
    fn search_target(nums: &[i32], left: bool, target: i32) -> i32 {
        // 获取目标值的极左或极右的坐标
        if nums.len() == 0 {
            return -1;
        }
        if nums.len() == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }
        // 其他情况
        let mid = (0 + nums.len() - 1) / 2;
        return if nums[mid] > target {
            Self::search_target(&nums[0..mid], left, target)
        } else if nums[mid] < target {
            let idx = Self::search_target(&nums[mid + 1..], left, target);
            if idx == -1 {
                -1
            } else {
                mid as i32 + idx + 1
            }
        } else {
            // nums[mid] == target
            if left {
                // 找极左值
                let idx = Self::search_target(&nums[0..mid], left, target);
                if idx == -1 {
                    mid as i32
                } else {
                    idx
                }
            } else {
                // 找极右值
                let idx = Self::search_target(&nums[mid + 1..], left, target);
                if idx == -1 {
                    mid as i32
                } else {
                    mid as i32 + idx + 1
                }
            }
        };
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j053() {
        assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 8), 2);
        println!("==========");
        assert_eq!(Solution::search(vec![8, 8, 8, 8, 8, 8], 8), 6);
        assert_eq!(Solution::search(vec![8], 8), 1);
        assert_eq!(Solution::search(vec![1, 2, 3], 8), 0);
        assert_eq!(Solution::search(vec![], 8), 0);
        assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 6), 0);

        assert_eq!(Solution::search_target(&[8, 8, 8], false, 8), 2)
    }
}
