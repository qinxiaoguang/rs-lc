pub struct Solution {}
impl Solution {
    // 判断一个数是不是回文数
    #![allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().rev().collect::<String>() == x.to_string()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l009() {
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(true, Solution::is_palindrome(0));
        assert_eq!(false, Solution::is_palindrome(-121));
    }
}
