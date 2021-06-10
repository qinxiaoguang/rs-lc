use super::Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        // 经典，完美数,与因子相关的，需要考虑 1-根号n
        let mut sum = 0;
        let mut i = 1;
        while i * i <= num {
            if num % i == 0 {
                sum += i;
                if i * i != num {
                    sum += num / i;
                }
            }
            i += 1;
        }
        sum == num * 2
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l507() {
        assert_eq!(Solution::check_perfect_number(28), true);
        assert_eq!(Solution::check_perfect_number(6), true);
        assert_eq!(Solution::check_perfect_number(496), true);
        assert_eq!(Solution::check_perfect_number(8128), true);
        assert_eq!(Solution::check_perfect_number(2), false);
        assert_eq!(Solution::check_perfect_number(1), false);
    }
}
