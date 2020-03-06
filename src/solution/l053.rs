pub struct Solution {}

impl Solution {
    // 最大的连续子数组的和
    // 要求O(n),也可以尝试O(logn)
    // 使用start,end双指针，当前子数组的和如果大于0，则end++,
    // 否则start++

    // 双指针，其实并不是简单的解法，简单的解法应该是..
    // for i in nums {
    //     if sum >= 0 { sum += i;} else {sum = i;}
    // }
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0_usize, 0_usize);
        let mut max = std::i32::MIN;
        let mut now_max = 0_i32;
        while end < nums.len() && start < nums.len() {
            if now_max < 0 {
                now_max -= nums[start];
                start += 1;
            } else {
                now_max += nums[end];
                max = std::cmp::max(max, now_max);
                end += 1;
            }
        }
        max
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l053() {
        println!(
            "{:?}",
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );

        println!("{:?}", Solution::max_sub_array(vec![1]));
        println!("{:?}", Solution::max_sub_array(vec![0]));
        println!("{:?}", Solution::max_sub_array(vec![]));
        println!("{:?}", Solution::max_sub_array(vec![-1]));
    }
}
