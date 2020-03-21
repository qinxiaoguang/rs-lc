use std::collections::HashMap;
pub struct Solution {}

// map macro
macro_rules! map {
    ($($k:expr => $v:expr),*) => {
        {
            let map = HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
}

impl Solution {
    #[allow(dead_code)]
    // 给你一个字符串 S、一个字符串 T，请在字符串 S 里面找出：包含 T 所有字母的最小子串。
    // 滑动窗口，start,end均滑动，start在退出时，也要做一些比较
    pub fn min_window(s: String, t: String) -> String {
        let mut t_map = map![];
        let mut min_len = std::usize::MAX;
        let mut res = String::from("");
        t.chars().for_each(|c| {
            *t_map.entry(c).or_insert(0_usize) += 1;
        });
        let s: Vec<char> = s.chars().collect();
        let mut window = map![];
        let mut end = 0_usize;
        for start in 0..s.len() {
            if !t_map.contains_key(&s[start]) {
                continue;
            }
            let window_entry = window.entry(s[start]).or_insert(0_usize);
            let t_map_entry = t_map.entry(s[start]).or_insert(0_usize);
            if *window_entry >= *t_map_entry && Self::is_equal(&t_map, &window) {
                // 去掉start后
                if min_len > end - start {
                    min_len = end - start;
                    let mut tmp = String::from("");
                    s[start..end].iter().clone().for_each(|c| tmp.push(*c));
                    res = tmp;
                }
                *window.entry(s[start]).or_insert(0_usize) -= 1;
                //println!("=={},{}", start, end);
                continue;
            }

            for j in end..s.len() {
                //println!("{},{}", start, end);
                end = j + 1;
                if !t_map.contains_key(&s[j]) {
                    continue;
                }
                let window_entry = window.entry(s[j]).or_insert(0_usize);
                let t_map_entry = t_map.entry(s[j]).or_insert(0_usize);
                *window_entry += 1;
                if *window_entry >= *t_map_entry && Self::is_equal(&t_map, &window) {
                    if min_len > end - start {
                        min_len = end - start;
                        let mut tmp = String::from("");
                        s[start..end].iter().clone().for_each(|c| tmp.push(*c));
                        res = tmp;
                    }
                    break;
                }
            }
            let window_entry = window.entry(s[start]).or_insert(0_usize);
            *window_entry -= 1;
        }
        res
    }

    fn is_equal(map1: &HashMap<char, usize>, map2: &HashMap<char, usize>) -> bool {
        //println!("{:?},{:?}", map1, map2);
        for (k, v) in map1.iter() {
            match map2.get(k) {
                Some(v2) => {
                    if v <= v2 {
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
    fn test_l076() {
        assert_eq!(
            "BANC",
            Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC"))
        );
        assert_eq!(
            "",
            Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABCZ"))
        );

        assert_eq!(
            "",
            Solution::min_window(String::from(""), String::from("ABCZ"))
        );

        assert_eq!(
            "",
            Solution::min_window(String::from("ABC"), String::from(""))
        );

        assert_eq!(
            "abbbbbcdd",
            Solution::min_window(String::from("aaaaaaaaaaaabbbbbcdd"), String::from("abcdd"))
        );

        assert_eq!(
            "bca",
            Solution::min_window(String::from("adobecodebancbbcaa"), String::from("abc"))
        );
    }
}
