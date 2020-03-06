pub struct Solution {}
impl Solution {
    // 原地删除排序数组中重复的数
    // 其实该题的意思是用两个指针，第一个指针指向当前需要被填充的位置，第二个位置指向后边判断的数的位置
    // 如果第二个位置的数和第一个位置内的数相等，就不填冲，否则，第一指针往后遍历并填冲。
    #![allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut first, mut second) = (0, 1);
        loop {
            if second >= nums.len() {
                break;
            }
            if nums[first] != nums[second] {
                first += 1;
                nums[first] = nums[second];
            }
            second += 1;
        }
        ((first + 1)..nums.len()).for_each(|_| {
            nums.remove(first + 1);
        });
        nums.len() as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l026() {
        assert_eq!(2, Solution::remove_duplicates(&mut vec![1, 1, 2]));
        assert_eq!(
            5,
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
        );
        assert_eq!(0, Solution::remove_duplicates(&mut vec![]));
        assert_eq!(1, Solution::remove_duplicates(&mut vec![1]));
    }
}
