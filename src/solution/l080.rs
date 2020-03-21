pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 原地修改重复出现的数组，并返回修改后的长度
    // 还是使用双指针
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut start = 2_usize;
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        for i in 2..nums.len() {
            if nums[i] == nums[start - 2] {
                continue;
            }
            nums.swap(i, start);
            start += 1;
        }
        for _ in start..nums.len() {
            nums.remove(start);
        }
        nums.len() as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l080() {
        let mut num = vec![1, 1, 1, 1, 2, 2, 3];
        Solution::remove_duplicates(&mut num);
        assert_eq!(vec![1, 1, 2, 2, 3], num);

        let mut num = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        Solution::remove_duplicates(&mut num);
        assert_eq!(vec![0, 0, 1, 1, 2, 3, 3], num);
    }
}
