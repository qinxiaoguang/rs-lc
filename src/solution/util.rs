use super::ListNode;
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

// set macro
macro_rules! list {
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
