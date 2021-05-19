use super::Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut res = vec![];
        // 此题滑动窗口可解
        let mut win = vec![0; 26];
        let (mut s, mut p): (Vec<usize>, Vec<usize>) = (
            s.chars().map(|c| c as usize - 'a' as usize).collect(),
            p.chars().map(|c| c as usize - 'a' as usize).collect(),
        );
        let win_len = p.len();
        if s.len() < win_len {
            return res;
        }
        let mut p_win = vec![0; 26];
        for n in p {
            p_win[n] += 1;
        }
        for n in &s[0..win_len] {
            win[*n] += 1;
        }
        if win.eq(&p_win) {
            res.push(0i32);
        }
        for i in win_len..s.len() {
            win[s[i - win_len]] -= 1;
            win[s[i]] += 1;
            if win.eq(&p_win) {
                res.push((i - win_len + 1) as i32);
            }
        }

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_438() {
        assert_eq!(
            Solution::find_anagrams(String::from("cbaebabacd"), String::from("abc")),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams(String::from("abab"), String::from("ab")),
            vec![0, 1, 2]
        );

        assert_eq!(
            Solution::find_anagrams(String::from("abab"), String::from("ababa")),
            vec![]
        );
    }
}
