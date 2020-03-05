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
