use super::Solution;

impl Solution {
    // 找到最左侧第一个下标不等于其值的数
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let res = Self::missing_number_with_idx(&nums, 0);
        if res == -1 {
            return nums.len() as i32;
        } else {
            return res;
        }
    }

    fn missing_number_with_idx(nums: &[i32], from: usize) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        if nums.len() == 1 {
            return if nums[0] == from as i32 {
                -1
            } else {
                nums[0] - 1
            };
        }
        let mid = (nums.len() - 1) / 2;
        if (from + mid) == nums[mid] as usize {
            // 答案在右侧
            Self::missing_number_with_idx(&nums[mid + 1..], from + mid + 1)
        } else {
            // 答案在左侧
            let res = Self::missing_number_with_idx(&nums[..mid], from);
            if res == -1 {
                nums[mid] - 1
            } else {
                res
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j053v2() {
        assert_eq!(Solution::missing_number(vec![0, 1, 2, 3, 5]), 4);
        assert_eq!(Solution::missing_number(vec![1]), 0);
        assert_eq!(Solution::missing_number(vec![0, 1, 3]), 2);
        assert_eq!(Solution::missing_number(vec![0]), 1);
        assert_eq!(Solution::missing_number(vec![0, 1, 2, 3, 4]), 5);
        assert_eq!(Solution::missing_number(vec![1, 2]), 0);
        assert_eq!(
            Solution::missing_number(vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
                45, 46, 47, 48, 49
            ]),
            18
        );
    }
}
