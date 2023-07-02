use std::ops::Index;

use super::Solution;

impl Solution {
    // 二进制加法
    pub fn add_binary(a: String, b: String) -> String {
        let max_len = if a.len() > b.len() { a.len() } else { b.len() };
        let (achars, bchars): (Vec<char>, Vec<char>) =
            (a.chars().rev().collect(), b.chars().rev().collect());
        let mut res = String::from("");
        let mut add_one = 0; // 进位
        for i in 0..max_len {
            let a = if achars.len() > i { achars[i] } else { '0' };
            let b = if bchars.len() > i { bchars[i] } else { '0' };
            let put = a as i32 - '0' as i32 + b as i32 - '0' as i32 + add_one;
            add_one = put / 2;
            res.push(if put % 2 == 0 { '0' } else { '1' });
        }
        if add_one == 1 {
            res.push('1');
        }

        res.chars().rev().collect()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_002() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "10".to_string()),
            "101".to_string()
        );
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
        assert_eq!(
            Solution::add_binary("1".to_string(), "1111".to_string()),
            "10000".to_string()
        );
    }
}
