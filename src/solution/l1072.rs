use super::Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        // 将每个数和每个数的反，放入hash中，最后hash中谁的值最大，谁就是结果
        // key多的，那么将这个key反转成相同数的时候，其他的key也必将被反转成相同的数
        let mut m = std::collections::HashMap::new();
        for item in matrix {
            let mut s = String::from("");
            let mut rev_s = String::from("");
            for i in item {
                if i == 0 {
                    s.push('0');
                    rev_s.push('1');
                } else {
                    s.push('1');
                    rev_s.push('0');
                }
            }
            *m.entry(s).or_insert(0) += 1;
            *m.entry(rev_s).or_insert(0) += 1;
        }
        *m.values().max().unwrap()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1072() {
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
            2
        );

        assert_eq!(Solution::max_equal_rows_after_flips(vec![vec![0]]), 1);
        assert_eq!(
            Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]),
            2
        );
    }
}
