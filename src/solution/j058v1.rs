use super::Solution;

impl Solution {
    // 先整体反转，再针对每个单词做反转
    pub fn reverse_words(s: String) -> String {
        let s: String = s.chars().rev().collect();
        let res: Vec<String> = s
            .split_whitespace()
            .map(|x| x.chars().rev().collect::<String>())
            .collect();
        res.join(" ")
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j058v1() {
        assert_eq!(Solution::reverse_words(s!("i am  blue")), s!("blue am i"));
    }
}
