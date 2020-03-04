pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    // 输出串联子串的位置，可以乱序，但不可重复
    // 采用滑动窗口,滑动窗口按照步长进行前进，注意要点是字符串的长度都是相同的
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res = vec![];
        if words.is_empty() {
            return res;
        }
        let word_len = words.first().unwrap().len();
        let mut words_map: HashMap<&str, i32> = HashMap::new();
        words.iter().for_each(|word| {
            *words_map.entry(word).or_insert(0_i32) += 1;
        });
        let words_len = s.len();
        for start in 0..words_len - words.len() * word_len + 1 {
            let mut window_map = HashMap::new();
            for end in (start..words_len - word_len + 1).step_by(word_len) {
                let word = &s[end..end + word_len];
                let now_count = window_map.entry(word).or_insert(0_i32);
                *now_count += 1;
                let count = words_map.get(word).unwrap_or(&0_i32);
                if *now_count < *count {
                    continue;
                }
                if *now_count > *count {
                    break;
                }
                if *now_count == *count {
                    // 判断 当前的map和Words_map是否相等，若相等，判断当前的window_map和words_map是否相等
                    // 若不等，则继续，若相等，则将start加入结果。
                    if Self::is_equal(&words_map, &window_map) {
                        res.push(start as i32);
                        break;
                    }
                }
            }
        }
        res
    }

    fn is_equal(map1: &HashMap<&str, i32>, map2: &HashMap<&str, i32>) -> bool {
        for (k, v) in map1.iter() {
            match map2.get(k) {
                Some(v2) => {
                    if v == v2 {
                        continue;
                    } else {
                        return false;
                    }
                }
                None => {
                    return false;
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
    fn test_l030() {
        assert_eq!(
            vec![0, 9],
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            )
        );
        assert_eq!(
            vec![8],
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            )
        );

        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "lingmindraboofooowingdingbarrwingmonkeypoundcake".to_string(),
                vec![
                    "fooo".to_string(),
                    "barr".to_string(),
                    "wing".to_string(),
                    "ding".to_string(),
                    "wing".to_string()
                ]
            ),
            vec![13]
        )
    }
}
