use super::Solution;

impl Solution {
    // 需要计算除i下标元素外，其他元素的乘积，还不能用除法
    // 一看就是前缀和(积)
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        let (mut l, mut r) = (vec![1; a.len()], vec![1; a.len()]);
        a.iter().enumerate().for_each(|(i, v)| {
            l[i] = if i >= 1 { l[i - 1] } else { 1 } * v;
        });
        a.iter().rev().enumerate().for_each(|(i, v)| {
            r[i] = if i >= 1 { r[i - 1] } else { 1 } * v;
        });

        let mut res = vec![0; a.len()];
        for i in 0..a.len() {
            res[i] = if i >= 1 { l[i - 1] } else { 1 }
                * if i == (a.len() - 1) {
                    1
                } else {
                    r[a.len() - i - 2]
                };
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j066() {
        assert_eq!(Solution::construct_arr(vec![2, 2, 2]), vec![4, 4, 4]);
        assert_eq!(
            Solution::construct_arr(vec![1, 2, 3, 4, 5]),
            vec![120, 60, 40, 30, 24]
        );
        assert_eq!(Solution::construct_arr(vec![]), vec![]);
    }
}
