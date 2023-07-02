use super::Solution;

impl Solution {
    // 找升序数组中和为目标数的两个数的下标，题目假设答案只有一个
    pub fn two_sum_v2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        while i < j {
            let sum = numbers[i] + numbers[j];
            if sum == target {
                return vec![i as i32, j as i32];
            }
            if sum > target {
                j -= 1;
                continue;
            }
            i += 1;
        }
        return vec![];
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_006() {
        assert_eq!(Solution::two_sum_v2(vec![1, 2, 4, 6, 10], 8), vec![1, 3]);
        assert_eq!(Solution::two_sum_v2(vec![2, 3, 4], 6), vec![0, 2]);
    }
}
