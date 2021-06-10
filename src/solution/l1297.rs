use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        // max_size完全没用，因为max_size能找到的答案，min_size也必须能找到，所以其实只需要找min_size的答案即可
        use std::collections::HashMap;
        let mut m = HashMap::new();
        let mut letters = HashMap::new();
        let s: Vec<char> = s.chars().collect();
        let min_size = min_size as usize;
        if min_size > s.len() {
            return 0;
        }
        for i in 0..min_size {
            *letters.entry(s[i]).or_insert(0) += 1;
        }
        let tmp: String = s[0..min_size].iter().collect();
        let lsize = letters.len();
        if lsize <= max_letters as usize {
            *m.entry(tmp).or_insert(0) += 1;
        }
        for i in min_size..s.len() {
            *letters.entry(s[i]).or_insert(0) += 1;
            let v = letters.get(&s[i - min_size]).unwrap();
            if *v == 1 {
                letters.remove(&s[i - min_size]);
            } else {
                letters.insert(s[i - min_size], *v - 1);
            }
            let tmp: String = s[i - min_size + 1..=i].iter().collect();
            if letters.len() <= max_letters as usize {
                *m.entry(tmp).or_insert(0) += 1;
            }
        }
        if m.len() == 0 {
            0
        } else {
            *m.values().into_iter().max().unwrap()
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1297() {
        assert_eq!(Solution::max_freq(String::from("aababcaab"), 2, 3, 4), 2);
        assert_eq!(Solution::max_freq(String::from("aaaa"), 1, 3, 3), 2);
        assert_eq!(Solution::max_freq(String::from("aabcabcab"), 2, 2, 3), 3);
        assert_eq!(Solution::max_freq(String::from("abcde"), 2, 3, 3), 0);
    }
}
