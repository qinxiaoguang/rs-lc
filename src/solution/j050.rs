use super::Solution;

impl Solution {
    // 必须要完整的遍历一次
    pub fn first_uniq_char(s: String) -> char {
        let mut m = std::collections::HashMap::new();
        s.chars().for_each(|c| {
            *m.entry(c).or_insert(0) += 1;
        });
        s.chars()
            .find(|c| *m.get(c).unwrap_or(&0) == 1)
            .unwrap_or(' ')
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j050() {
        assert_eq!(Solution::first_uniq_char(s!("abcab")), 'c');
    }
}
