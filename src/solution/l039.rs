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
    // 给排序的非重复的数，找出所有的子数组相加等于target的数组，其中每个数可以被无限制的重复
    // 以为有什么黑科技的方法，实际上只是用sort + dfs+剪枝就能解决
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut candidates = candidates;
        candidates.sort();
        Self::backtracking(&candidates, target, &mut res, &mut vec![]);
        res
    }

    fn backtracking(condidates: &[i32], target: i32, res: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>) {
        //println!("{:?},{},{:?},{:?}", condidates, target, res, tmp);
        for (k, &i) in condidates.iter().enumerate() {
            if target - i > 0 {
                tmp.push(i);
                Self::backtracking(&condidates[k..], target - i, res, tmp);
                tmp.pop();
                continue;
            }
            if target - i == 0 {
                tmp.push(i);
                res.push(tmp.clone());
                tmp.pop();
                continue;
            }
            return;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l039() {
        println!("{:?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
        println!("{:?}", Solution::combination_sum(vec![2, 3, 5], 8));
        println!("{:?}", Solution::combination_sum(vec![2, 3, 5], 0));
        println!("{:?}", Solution::combination_sum(vec![], 8));
    }
}
