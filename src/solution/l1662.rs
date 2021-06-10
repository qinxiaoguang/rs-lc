use super::Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1662() {
        assert_eq!(
            Solution::array_strings_are_equal(sv!["a", "bc"], sv!["ab", "c"]),
            true
        );
    }
}
