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
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0_i32;
        }
        let mut left = vec![0; height.len()];
        let mut right = vec![0; height.len()];
        for i in 1..height.len() {
            left[i] = std::cmp::max(left[i - 1], height[i - 1]);
        }
        for i in (0..height.len() - 1).rev() {
            right[i] = std::cmp::max(right[i + 1], height[i + 1]);
        }

        let mut water = 0_i32;
        for i in 0..height.len() {
            let min = std::cmp::min(left[i], right[i]);
            water += if height[i] > min { 0 } else { min - height[i] };
        }
        water
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l042() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
        assert_eq!(0, Solution::trap(vec![0, 1]));
        assert_eq!(0, Solution::trap(vec![0]));
        assert_eq!(1, Solution::trap(vec![1, 0, 1]));
        assert_eq!(0, Solution::trap(vec![1, 1, 1]));
        assert_eq!(0, Solution::trap(vec![]));
    }
}
