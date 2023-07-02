use super::Solution;
use super::*;
use std::collections::HashMap;

impl Solution {
    // 将出现次数相同的字符串分到一组
    // 最简单的排序
    // 再优化一点就是空间换时间
    // 再优化就是利用质数表相乘获取key
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans = vec![];
        let mut m = HashMap::new();
        let prime = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
            97, 101, 103,
        ];
        strs.into_iter().for_each(|str| {
            let k = str
                .as_bytes()
                .iter()
                .fold(1i64, |m, x| m * prime[(*x - b'a') as usize]);
            (*m.entry(k).or_insert(vec![])).push(str);
        });
        for v in m.into_values() {
            ans.push(v);
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_033() {
        let res = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
        ]);
        println!("{:?}", res);

        let res = Solution::group_anagrams(vec!["".to_string()]);
        println!("{:?}", res);
    }
}
