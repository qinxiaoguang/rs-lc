use super::Solution;

impl Solution {
    // 可以调用语言的库来实现，但该题的题意是想让你手动实现
    // 方法也很简单，就是创建新数组,慢慢遍历即可
    pub fn replace_space(s: String) -> String {
        let cs: Vec<char> = s.chars().collect();
        let mut res = Vec::with_capacity(cs.len() * 3);
        for c in cs.into_iter() {
            match c {
                ' ' => res.append(&mut vec!['%', '2', '0']),
                c => res.push(c),
            };
        }
        res.into_iter().collect()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j005() {
        assert_eq!(Solution::replace_space(s!("a b")), s!("a%20b"));
    }
}
