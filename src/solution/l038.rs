pub struct Solution {}
// String::from == sf!
macro_rules! sf {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    #![allow(dead_code)]
    pub fn count_and_say(n: i32) -> String {
        let mut source_string = sf!("1");
        for _ in 0..n - 1 {
            let mut tmp_string = sf!("");
            let mut last: u8 = b' ';
            let mut count = 0;
            for b in source_string.as_bytes() {
                if b == &last {
                    count += 1;
                } else {
                    if count != 0 {
                        tmp_string.push_str(&format!("{}{}", count, last as char));
                    }
                    count = 1;
                    last = *b;
                }
            }
            tmp_string.push_str(&format!("{}{}", count, last as char));
            source_string = tmp_string;
        }
        source_string
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l038() {
        assert_eq!("1", Solution::count_and_say(1));
        assert_eq!("11", Solution::count_and_say(2));
        assert_eq!("21", Solution::count_and_say(3));
        assert_eq!("1211", Solution::count_and_say(4));
        assert_eq!("111221", Solution::count_and_say(5));
    }
}
