use super::*;

impl Solution {
    // 返回target在数组中的位置，如果不存在，则返回应该被插入的位置
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0usize, nums.len() - 1);

        while l < r {
            let mid = (l + r) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[mid] > target {
                r = mid;
                continue;
            }
            l = mid + 1;
        }
        if nums[l] == target {
            return l as i32;
        }
        return if nums[l] > target { l } else { l + 1 } as i32;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_068() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
        assert_eq!(Solution::search_insert(vec![1, 3], 0), 0);
    }
}
