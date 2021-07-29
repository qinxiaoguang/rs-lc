use super::Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let n: usize = n as usize % s.len();
        let mut s: Vec<char> = s.chars().collect();
        s[n..].reverse();
        s[..n].reverse();
        s.reverse();
        s.iter().collect()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j058v2() {
        assert_eq!(Solution::reverse_left_words(s!("abc"), 1), s!("bca"));
        assert_eq!(
            Solution::reverse_left_words(s!("abcdefg"), 2),
            s!("cdefgab")
        );
    }
}
