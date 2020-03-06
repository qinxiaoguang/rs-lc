pub struct Solution {}
impl Solution {
    // 查找一个排序数组中，目标值的最左位置和最右位置
    // 写得有点复杂，后续可以优化
    #![allow(dead_code)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![
            Self::find_left(&nums, target),
            Self::find_right(&nums, target),
        ]
    }

    fn find_left(nums: &[i32], target: i32) -> i32 {
        //println!("{:?}", nums);
        if nums.len() == 0 {
            return -1;
        }
        if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            } else {
                return -1;
            }
        }
        if nums.len() == 2 {
            if nums[0] == target {
                return 0;
            }
            if nums[1] == target {
                return 1;
            }
            return -1;
        }
        let mid = (0 as i32 + nums.len() as i32) / 2;
        if nums[mid as usize] >= target {
            if mid as i32 >= 0 {
                let left_res = Self::find_left(&nums[0 as usize..mid as usize + 1], target);
                if left_res != -1 {
                    return left_res;
                }
            }
        } else {
            if (mid as usize) < nums.len() {
                let right_res = Self::find_left(&nums[mid as usize + 1..nums.len()], target);
                if right_res != -1 {
                    return right_res + mid + 1;
                }
            }
        }
        -1
    }

    fn find_right(nums: &[i32], target: i32) -> i32 {
        //println!("{:?}", nums);
        if nums.len() == 0 {
            return -1;
        }
        if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            } else {
                return -1;
            }
        }
        if nums.len() == 2 {
            if nums[1] == target {
                return 1;
            }
            if nums[0] == target {
                return 0;
            }
            return -1;
        }
        let mid = (0 as i32 + nums.len() as i32) / 2;
        if nums[mid as usize] <= target {
            if (mid as usize) <= nums.len() {
                let right_res = Self::find_right(&nums[mid as usize..nums.len() as usize], target);
                if right_res != -1 {
                    return right_res + mid;
                }
            }
        } else {
            if (mid as usize) > 0 {
                let left_res = Self::find_right(&nums[0..mid as usize], target);
                if left_res != -1 {
                    return left_res;
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
    fn test_l034() {
        println!("{}", Solution::find_right(&vec![5, 7, 7, 8, 8, 10], 6));
    }
}
