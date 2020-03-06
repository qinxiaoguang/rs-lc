pub struct Solution {}

impl Solution {
    // 给定一个没有重复数字的数组，返回其所有的全排列
    #![allow(dead_code)]
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
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
                    Self::dfs(nums, used, res, tmp);
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
    fn test_l046() {
        println!("{:?}", Solution::permute(vec![1, 2, 3]));
        println!("{:?}", Solution::permute(vec![1]));
        println!("{:?}", Solution::permute(vec![]));
    }
}
