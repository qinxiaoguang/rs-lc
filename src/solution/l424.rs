use super::Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        use std::cmp::max;
        // 滑动窗口，有点难想出来，核心思路就是计算每个字符的次数，窗口中不关心字符的位置
        // 只关心字符的个数，如果有任意的一个字符个数 >= 窗口大小-k, 那么这个窗口就是符合条件的
        // 一直滑动窗口，滑动窗口的核心的逻辑就是找条件让右窗口滑动及左窗口滑动。
        // 需要保证的是滑动窗口的长度一直是当前解的长度
        let v: Vec<usize> = s
            .chars()
            .map(|c| (c as i32 - 'A' as i32) as usize)
            .collect();
        let mut maxn = 0;
        let mut cnt = vec![0; 26];
        let (mut start, mut end) = (0usize, 0usize);
        while end < v.len() {
            cnt[v[end]] += 1;
            maxn = max(maxn, cnt[v[end]]);
            if maxn + k < (end - start + 1) as i32 {
                cnt[v[start]] -= 1;
                // maxn不用更新的原因是，max+k已经是固定值了，所以是为了找下一个比maxn更大的数，即便不更新maxn也不影响结果
                start += 1;
            }
            end += 1;
        }
        (end - start) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l424() {
        assert_eq!(
            4,
            Solution::character_replacement(String::from("AABABBA"), 1)
        );

        assert_eq!(4, Solution::character_replacement(String::from("ABAB"), 2));
        assert_eq!(2, Solution::character_replacement(String::from("ABAA"), 0));
    }
}
