use super::Solution;

impl Solution {
    // 只需要记录进位即可
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut push = false; // 是否进位
        let mut k = k;
        let mut res: Vec<i32> = num
            .into_iter()
            .rev()
            .map(|mut x| {
                x += (k % 10 + if push { 1 } else { 0 });
                k /= 10;
                push = if x >= 10 { true } else { false };
                x % 10
            })
            .collect();
        res.reverse();
        k += if push { 1 } else { 0 };
        while k != 0 {
            res.insert(0, k % 10);
            k /= 10;
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l989() {
        assert_eq!(
            Solution::add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        assert_eq!(
            Solution::add_to_array_form(vec![2, 1, 5], 806),
            vec![1, 0, 2, 1]
        );
        assert_eq!(
            Solution::add_to_array_form(vec![2, 7, 4], 181),
            vec![4, 5, 5]
        );

        assert_eq!(Solution::add_to_array_form(vec![1], 181), vec![1, 8, 2]);
    }
}
