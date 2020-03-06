pub struct Solution {}

impl Solution {
    // 给定一个未排序的数组，找到其中没有出现的最小的正整数
    // 要求时间复杂度是o(n), 空间复杂度是o(1)
    // 所以其实是要求你在该数组上做修改并得到结果。
    // 如果不要求空间复杂度的话，其实很好解，假如一个数组长度是n,那么结果一定是0~n中的一个，所以使用arr[n]来记录没被遍历的数，最后找到arr中的最小值即可。
    // 但是现在要求空间复杂度为o(1)，那么其实就是在该数组上，将其转变为arr[n]
    // 方法就是从左往右遍历，将所有的数放到其正确的位置，正确位置的含义是，其坐标和其值是相等的，如当前遍历到的值是2,那么arr[2]=2,即这两个位置交换
    // 最终就是遍历一遍找到第一个不在正确位置的数。
    #![allow(dead_code)]
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 1;
        }
        let mut nums = nums;
        let mut index = 0_usize;
        let mut swap_index: i32;
        loop {
            //println!("{:?}", nums);
            if index >= nums.len() {
                break;
            }
            swap_index = nums[index];
            if swap_index >= nums.len() as i32
                || swap_index <= 0
                || swap_index == index as i32 + 1
                || nums[index] == nums[swap_index as usize - 1]
            {
                index += 1;
                continue;
            }
            nums.swap(index, swap_index as usize - 1);
        }
        //println!("{:?}", nums);
        let len = nums.len();
        nums.into_iter()
            .enumerate()
            .find(|(index, v)| *index as i32 + 1 != *v)
            .map(|(index, _)| index as i32 + 1)
            .unwrap_or(len as i32 + 1)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l041() {
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
        assert_eq!(1, Solution::first_missing_positive(vec![0]));
        assert_eq!(2, Solution::first_missing_positive(vec![1]));
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 10, 11]));
        assert_eq!(2, Solution::first_missing_positive(vec![1, 1]));
    }
}
