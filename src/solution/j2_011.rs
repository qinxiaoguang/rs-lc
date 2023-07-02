use super::Solution;

impl Solution {
    // 找到含有相同数量的01个数的最长连续子数组
    // 将0换为-1， 则题目变为找到和为0的最长连续子数组
    // 依然使用前缀和的方式求解，那么题目又变为，找到前缀和中两数相减为0的数
    // 使用hash表来存储数的最左索引,遍历的时候更新该hash表，并更新结果即可
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut m = std::collections::HashMap::with_capacity(nums.len());
        let mut sum = 0;
        m.insert(0, 0usize);
        nums.iter().enumerate().for_each(|(i, &x)| {
            sum += if x == 0 { -1 } else { x };
            // 找-sum
            if let Some(left) = m.get(&sum) {
                res = res.max(i + 1 - left);
            } else {
                m.insert(sum, i + 1);
            }
        });

        res as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_011() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
        assert_eq!(Solution::find_max_length(vec![0, 1, 0, 1]), 4);
        assert_eq!(Solution::find_max_length(vec![0, 1, 0, 1]), 4);
    }
}
