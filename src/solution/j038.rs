use super::Solution;

impl Solution {
    // 注意 需要与n & (n-1)进行区分，n & (n-1)是获取所有的子集
    // 全排列可以采用优先固定第一位，剩下的采用递归的方式获取剩余的全排列
    // 但是这种方式对于有重复字符的将会失效, 所以需要采用set来去掉重复的字符
    pub fn permutation(s: String) -> Vec<String> {
        if s.len() == 1 {
            return vec![s];
        }
        let mut set = std::collections::HashSet::new();
        let mut res = vec![];

        // 固定第一位
        let mut cs: Vec<char> = s.chars().collect();
        for i in 0..cs.len() {
            if !set.contains(&cs[i]) {
                set.insert(cs[i]);
                cs.swap(0, i);
                Self::permutation(cs[1..].iter().collect())
                    .into_iter()
                    .for_each(|mut s| {
                        s.insert(0, cs[0]);
                        res.push(s);
                    });
                cs.swap(0, i);
            }
        }

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j038() {
        println!(
            "Solution::permutation(s!(abc)) = {:?}",
            Solution::permutation(s!("abc"))
        );
        println!(
            "Solution::permutation(s!) = {:?}",
            Solution::permutation(s!("aab"))
        );
    }
}
