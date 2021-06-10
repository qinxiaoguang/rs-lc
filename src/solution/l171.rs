use super::Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        // 其实就是进制问题，每移动一个位，乘上26
        let mut res = 0;
        column_title.chars().into_iter().for_each(|c| {
            res = res * 26 + (c as i32 - 'A' as i32 + 1);
        });
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l171() {
        assert_eq!(Solution::title_to_number(String::from("AA")), 27);
        assert_eq!(Solution::title_to_number(String::from("A")), 1);
        assert_eq!(Solution::title_to_number(String::from("ZY")), 701);
    }
}
