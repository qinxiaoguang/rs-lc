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
    // 有重复的数组 找出不重复的全排列
    // 此题很经典 ，如果使用set去重，效率会非常低。
    // 而在dfs搜索的时候，其实可以使用剪枝的方法去重
    // 剪枝的条件其实很简单，首先要对一个sort后的数组进行dfs
    // 那么在dfs的时候，判断当前坐标的点与上个坐标的点是否相等，若相等，就不需要往下继续深搜遍历
    // 原因就是上个节点已经进行过了一次深搜遍历，比如[1,1,2],对第一个坐标1进行遍历后，再对第二个坐标的1进行遍历，就造成了重复
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums = nums;
        nums.sort_unstable(); // 这个采用快排,更快，不知道为什么叫这个名字 ，nums.sort()采用分治排序
        Self::dfs(&nums, &mut vec![false; nums.len()], &mut res, &mut vec![]);
        res
    }

    fn dfs(nums: &[i32], used: &mut [bool], res: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>) {
        if tmp.len() >= nums.len() {
            return;
        }
        for i in 0..nums.len() {
            if !used[i] {
                used[i] = true;
                tmp.push(nums[i]);
                if tmp.len() == nums.len() {
                    res.push(tmp.clone());
                } else {
                    // 剪枝
                    if i < 1 || nums[i] != nums[i - 1] || used[i - 1] {
                        Self::dfs(nums, used, res, tmp);
                    }
                }
                tmp.pop();
                used[i] = false;
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l047() {
        println!("{:?}", Solution::permute_unique(vec![1, 2, 3]));
        println!("{:?}", Solution::permute_unique(vec![1, 1, 3]));
    }
}
