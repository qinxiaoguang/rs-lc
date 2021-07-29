use super::Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut s = std::collections::HashSet::new();
        for n in nums {
            if s.contains(&n) {
                return n;
            }
            s.insert(n);
        }
        0
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j003() {
        assert_eq!(Solution::find_repeat_number(vec![1, 2, 3, 1]), 1);
    }
}
