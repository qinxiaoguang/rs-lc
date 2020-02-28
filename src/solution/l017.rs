pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    // 给出[2-9]的数字，输出所有的可能的手机按键输出的字符串
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut map = HashMap::new();
        map.insert('2', vec!['a', 'b', 'c']);
        map.insert('3', vec!['d', 'e', 'f']);
        map.insert('4', vec!['g', 'h', 'i']);
        map.insert('5', vec!['j', 'k', 'l']);
        map.insert('6', vec!['m', 'n', 'o']);
        map.insert('7', vec!['p', 'q', 'r', 's']);
        map.insert('8', vec!['t', 'u', 'v']);
        map.insert('9', vec!['w', 'x', 'y', 'z']);
        if digits.len() == 0 {
            return vec![];
        }
        let mut res: Vec<String> = vec![];
        let chars: Vec<char> = digits.chars().collect();
        Self::get_all_letter("", &chars, &map, &mut res);
        res
    }

    fn get_all_letter(
        now_letter: &str,
        chars: &[char],
        map: &HashMap<char, Vec<char>>,
        res: &mut Vec<String>,
    ) {
        if chars.len() == 0 {
            res.push(now_letter.to_string());
            return;
        }
        let c = chars[0];
        let v = map.get(&c).unwrap();
        for &i in v.iter() {
            let mut cloned_letter = now_letter.clone().to_string();
            cloned_letter.push(i);
            Self::get_all_letter(&cloned_letter, &chars[1..], map, res);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l017() {
        println!("{:?}", Solution::letter_combinations("23".to_string()));
        println!("{:?}", Solution::letter_combinations("".to_string()));
        println!("{:?}", Solution::letter_combinations("232".to_string()));
    }
}
