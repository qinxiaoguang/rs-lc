use super::Solution;

impl Solution {
    // 使用map
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        nums.iter().for_each(|n| {
            *m.entry(n).or_insert(0) += 1;
        });

        let mut res = vec![];
        nums.iter().find(|&n| {
            let last = target - n;
            return if *n != last && m.contains_key(&last) {
                res.push(*n);
                res.push(last);
                true
            } else if *n == last {
                // 两个相等
                if *m.entry(n).or_default() < 2 {
                    return false;
                }
                res.push(*n);
                res.push(*n);
                true
            } else {
                false
            };
        });
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j057() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![2, 7]);
        assert_eq!(Solution::two_sum(vec![3, 3, 4, 4], 6), vec![3, 3]);
        assert_eq!(
            Solution::two_sum(vec![10, 18, 25, 33, 36, 50, 50, 52, 57, 74], 126),
            vec![52, 74]
        );
    }
}
