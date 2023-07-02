use super::Solution;

impl Solution {
    // 找到s中所有p的排列组合的子串位置
    // 依然是滑动窗口解
    pub fn find_anagrams_v2(s: String, p: String) -> Vec<i32> {
        let mut target = [0; 27];
        let mut source = [0; 27];
        let (mut left, mut right) = (0, p.len());

        p.chars()
            .for_each(|c| source[c as usize - 'a' as usize] += 1);
        let s2: Vec<char> = s.chars().collect();
        s2.iter()
            .enumerate()
            .filter(|(i, c)| {
                if *i >= p.len() {
                    target[s2[*i - p.len()] as usize - 'a' as usize] -= 1;
                }
                target[**c as usize - 'a' as usize] += 1;
                target == source
            })
            .map(|x| (x.0 + 1 - p.len()) as i32)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_015() {
        assert_eq!(
            Solution::find_anagrams_v2("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );

        assert_eq!(
            Solution::find_anagrams_v2("abab".to_string(), "ab".to_string()),
            vec![0, 1, 2]
        );
    }
}
