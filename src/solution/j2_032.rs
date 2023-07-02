use super::Solution;
use super::*;
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s == t {
            return false;
        }
        let mut m = HashMap::new();
        s.chars().for_each(|x| *m.entry(x).or_insert(0) += 1);
        let t = t.chars().collect::<Vec<char>>();

        for c in t.iter() {
            if !m.contains_key(c) {
                return false;
            }
            let mut cnt = m.entry(*c).or_insert(0);
            *cnt -= 1;
            if *cnt < 0 {
                return false;
            }
        }

        m.iter().find(|(k, v)| **v != 0).is_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_032() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
    }
}
