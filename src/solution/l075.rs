pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 对数组中的红、白、蓝 进行排序，不允许使用排序函数
    // 方法其实很简单，就是用三指针，前后指针分别当前应该当插入的位置,再加上一个遍历指针
    // 注意点是，遍历不是单纯的一直往后遍历，而是在交换数之后，交换后的数可能还需要交换，所以交换后，当前遍历点不能自增1
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut left, mut right) = (0_usize, nums.len() - 1);
        let mut cur = 0_usize;
        while cur < nums.len() {
            match nums[cur] {
                0 if left < cur => {
                    nums.swap(left, cur);
                    left += 1;
                }
                2 if right > cur => {
                    nums.swap(cur, right);
                    right -= 1;
                }
                _ => {
                    cur += 1;
                }
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l075() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);

        let mut nums = vec![0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0], nums);

        let mut nums = vec![0, 0, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 0, 0], nums);

        let mut nums = vec![2, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![2, 2, 2], nums);

        let mut nums = vec![0, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 2], nums);

        let mut nums = vec![2, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 2], nums);

        let mut nums = vec![1, 2, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(vec![0, 1, 2], nums);
    }
}
