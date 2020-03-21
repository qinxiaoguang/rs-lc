pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    //给定两个整数 n 和 k，返回 1 ... n 中所有可能的 k 个数的组合。
    // 注意是k个数的组合
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut used = vec![false; n as usize + 1];
        let mut res = vec![];
        Self::dfs(&mut used, &mut res, &mut vec![], k as usize, 1);
        res
    }

    fn dfs(
        used: &mut Vec<bool>,
        res: &mut Vec<Vec<i32>>,
        tmp: &mut Vec<i32>,
        k: usize,
        index: usize,
    ) {
        if tmp.len() == k {
            res.push(tmp.clone());
        }
        for i in index..used.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            tmp.push(i as i32);
            Self::dfs(used, res, tmp, k, i);
            tmp.pop();
            used[i] = false;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l077() {
        for i in 0..10 {
            for j in 0..10 {
                println!("{:?}", Solution::combine(i, j));
            }
        }
    }
}
