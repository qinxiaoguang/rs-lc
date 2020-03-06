pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 返回最后一个单词的长度
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap_or("").len() as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l058() {
        println!("{}", Solution::length_of_last_word("heihei".to_string()));
    }
}
