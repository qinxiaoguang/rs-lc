use super::Solution;
use super::*;
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut re_order = vec![0; 26];
        order
            .as_bytes()
            .into_iter()
            .enumerate()
            .for_each(|(i, c)| re_order[(*c - b'a') as usize] = i + 1);

        let max_len = words.iter().map(|x| x.len()).max().unwrap_or(0);

        let words: Vec<&[u8]> = words.iter().map(|x| x.as_bytes()).collect();
        // 两个单词两个单词的对比
        let mut now_word = words[0];

        for i in 1..words.len() {
            for j in 0..now_word.len() {
                if j >= words[i].len() {
                    return false;
                }
                let pre = re_order[(now_word[j] - b'a') as usize];
                let now = re_order[(words[i][j] - b'a') as usize];
                if pre > now {
                    return false;
                }
                if pre == now {
                    continue;
                }
                if pre < now {
                    now_word = words[i];
                    break;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_034() {
        assert_eq!(
            Solution::is_alien_sorted(
                vec!["hello".to_string(), "leetcode".to_string()],
                "hlabcdefgijkmnopqrstuvwxyz".to_string()
            ),
            true
        );

        assert_eq!(
            Solution::is_alien_sorted(
                vec!["word".to_string(), "world".to_string(), "row".to_string()],
                "worldabcefghijkmnpqstuvxyz".to_string()
            ),
            false
        );
    }
}
