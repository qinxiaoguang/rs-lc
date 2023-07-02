use super::Solution;

impl Solution {
    // 判断s2中是否包含s1的某个排列
    // 滑动窗口，并使用hash表来进行判断，只要hash表中的数据一样，就相等
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut target = [0; 27];
        let mut source = [0; 27];
        let (mut left, mut right) = (0, s1.len());

        s1.chars()
            .for_each(|c| source[c as usize - 'a' as usize] += 1);
        s2.chars()
            .take(s1.len())
            .for_each(|c| target[c as usize - 'a' as usize] += 1);

        if target == source {
            return true;
        }

        let s2: Vec<char> = s2.chars().collect();
        s2.iter()
            .skip(s1.len())
            .enumerate()
            .find(|(i, c)| {
                target[s2[*i] as usize - 'a' as usize] -= 1;
                target[**c as usize - 'a' as usize] += 1;
                target == source
            })
            .map(|x| true)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_014() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );

        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "a".to_string()),
            false
        );

        assert_eq!(
            Solution::check_inclusion("adc".to_string(), "dcda".to_string()),
            true
        );
    }
}
