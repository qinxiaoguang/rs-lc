use super::Solution;
use std::cmp::Ordering::Equal;

impl Solution {
    // 返回拼接结果中最小值的字符串
    // 其实就是字母排序,但是假设有两个字符串，一个是80，一个是807， 此时最小值应该是80780
    // 但如果一个是80，一个是809， 你会发现，最小值会是80809，而不是80980
    // 所以这两个货排序的规则，80要补充成和809一样的三个字符串，再排序
    // 补充方式就是重复前边的字符串，如80，变成重复的就是808080
    // 而变成3个字符串，就成了808，所以808和809进行排序，就能知道谁排前，谁排后
    // 所以需要自定义排序
    pub fn min_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.iter().map(i32::to_string).collect();
        nums.sort_by(|a, b| a.repeat(b.len()).cmp(&b.repeat(a.len())));
        nums.join("")
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j045() {
        assert_eq!(Solution::min_number(vec![3, 30, 34, 5, 9]), s!("3033459"));
        assert_eq!(Solution::min_number(vec![10, 2]), s!("102"));
        assert_eq!(Solution::min_number(vec![10, 1]), s!("101"));
        assert_eq!(Solution::min_number(vec![12, 121]), s!("12112"));
        assert_eq!(
            Solution::min_number(vec![824, 938, 1399, 5607, 6973, 5703, 9609, 4398, 8247]),
            s!("1399439856075703697382478249389609")
        );
    }
}
