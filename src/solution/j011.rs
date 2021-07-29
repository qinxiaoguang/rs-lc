use super::Solution;

impl Solution {
    // 求最小值
    // 只需要比较中间数和最后一个数的关系，即可知道最小值在左侧还是右侧
    // 以此为条件进行二分，但需要注意的是会有重复的数
    // 所以当中间数和最后一个数相等时,无法知道最小值在最左侧还是最右侧(假如只有两个数的情况)
    // 将最后一个数忽略掉，即end -= 1继续进行比较
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        if numbers.len() == 0 {
            return -1;
        }
        let (mut l, mut r) = (0, numbers.len() - 1);
        while l < r {
            let mid = (l + r) / 2;
            if numbers[mid] > numbers[r] {
                // 答案不可能是numbers[mid],所以将mid抛出，使用mid+1
                l = mid + 1;
            } else if numbers[mid] < numbers[r] {
                // 答案有可能是mid,所以mid不能-1
                r = mid;
            } else {
                r -= 1;
            }
        }

        numbers[l]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j011() {
        assert_eq!(Solution::min_array(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::min_array(vec![2, 2, 2, 0, 1]), 0);
        assert_eq!(Solution::min_array(vec![2, 2, 2, 1, 1]), 1);
    }
}
