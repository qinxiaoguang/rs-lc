pub struct Solution {}
impl Solution {
    // 原地移除所有等于val的数，返回数组的长度
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut first, mut second) = (0, 0);
        loop {
            if second >= nums.len() {
                break;
            }
            if val != nums[second] {
                nums[first] = nums[second];
                first += 1;
            }
            second += 1;
        }
        (first..nums.len()).for_each(|_| {
            nums.remove(first);
        });
        nums.len() as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l027() {
        assert_eq!(2, Solution::remove_element(&mut vec![3, 2, 2, 3], 3));
        assert_eq!(0, Solution::remove_element(&mut vec![], 3));
        assert_eq!(0, Solution::remove_element(&mut vec![3, 3, 3, 3, 3, 3], 3));
        assert_eq!(
            5,
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2)
        );
        assert_eq!(
            1,
            Solution::remove_element(&mut vec![2, 3, 3, 3, 3, 3, 3], 3)
        );
    }
}
