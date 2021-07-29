use super::Solution;

impl Solution {
    // 利用 a异或b来解
    // 要把a和b分到两个不同的组里面
    // 首先获取a 异或b的值，假设为z
    // 则从z中找到一位为1的值，则以此位为目标进行分组
    // 遍历数组，若该位为1，分为左不的组，若为0，则分到右边的组
    // 这样就能保证相同的数肯定能分到一个组里边，且ab在不同组里
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let num: i32 = nums.iter().fold(0i32, |res, x| res ^ *x);
        let mut cnt = 0;
        while (num >> cnt) & 1 == 0 {
            cnt += 1;
        }

        let mut left = 0;
        let mut right = 0;
        nums.iter().for_each(|n| {
            if (*n >> cnt) & 1 == 1 {
                left ^= *n;
            } else {
                right ^= *n;
            }
        });
        vec![left, right]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j056v1() {
        assert_eq!(0 ^ 100, 100);
        assert_eq!(
            Solution::single_numbers(vec![1, 2, 10, 4, 1, 4, 3, 3]),
            vec![10, 2]
        );
        assert_eq!(Solution::single_numbers(vec![4, 1, 4, 6]), vec![1, 6]);
        assert_eq!(
            Solution::single_numbers(vec![6, 5, 5, 9, 10, 9, 4, 10]),
            vec![6, 4]
        );
    }
}
