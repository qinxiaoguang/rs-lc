pub struct Solution {}
use std::collections::{HashMap, HashSet};
// String::from == sf!
macro_rules! sf {
    ($str:expr) => {
        String::from($str)
    };
}

// map macro
macro_rules! map {
    ($($k:expr => $v:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
}

// set macro
macro_rules! set {
    ($($k:expr),*) => {
        {
            let mut set = HashSet::new();
            $(
                map.insert($k);
            )*
            set
        }
    };
}

impl Solution {
    // 给定一个字符串数组，将字母异位词组合在一起。字母异位词指字母相同，但排列不同的字符串。
    // 一个很巧妙的算法是将a-z 用质数来表示，不同质数的乘积必定不同，利用此原理进行分组,很巧妙，很经典
    #![allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let prime = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101,
        ];
        let mut m = map![];
        let mut v = vec![];
        for s in strs {
            let mut times = 1;
            for b in s.as_bytes() {
                times *= prime[(b - b'a') as usize];
            }
            (*m.entry(times).or_insert(vec![])).push(s);
        }
        for (_, value) in m {
            v.push(value);
        }
        v
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l049() {
        println!(
            "{:?}",
            Solution::group_anagrams(vec![
                sf!("eat"),
                sf!("tea"),
                sf!("tan"),
                sf!("ate"),
                sf!("nat"),
                sf!("bat")
            ])
        );
        println!(
            "{:?}",
            Solution::group_anagrams(vec![sf!("ape"), sf!("pea"), sf!("tax"),])
        );
    }
}
