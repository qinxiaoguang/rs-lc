use super::Solution;

impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        // 返回这玩意的最大值|arr1[i] - arr1[j]| + |arr2[i] - arr2[j]| + |i - j|
        // 这玩意 的结果有下列8种
        // arr1[i] - arr1[j] + arr2[i] - arr2[j] + i - j = (arr1[i] + arr2[i] + i) - (arr1[j]+arr2[j]+j)
        // arr1[i] - arr1[j] + arr2[j] - arr2[i] + i - j = (arr1[i] - arr2[i] + i) - (arr2[j] - arr2[j] + j )
        // arr1[i] - arr1[j] + arr2[i] - arr2[j] + j - i = (arr1[i] + arr2[i] - i) - (arr1[i] + arr2[j] - j )
        // arr1[i] - arr1[j] + arr2[j] - arr2[i] + j - i = (arr1[i] - arr2[i] - i) - (arr1[j] - arr2[j] - j )
        // arr1[j] - arr1[i] + arr2[i] - arr2[j] + i - j = (arr2[i] - arr1[i] + i) - (arr2[j] - arr1[j] + j)
        // arr1[j] - arr1[i] + arr2[j] - arr2[i] + i - j = (i-arr1[i]-arr2[i]) -  (j-arr1[j] - arr2[j])
        // arr1[j] - arr1[i] + arr2[i] - arr2[j] + j - i = (arr2[i]-arr1[i] - i) - (arr2[j] - arr1[j] - j)
        // arr1[j] - arr1[i] + arr2[j] - arr2[i] + j - i = (arr1[j] + arr2[j]+j) - (arr1[i] + arr2[i] + i )
        // 找规律只需要求4个数组
        //arr1[i] + arr2[i] + i
        //arr1[i] - arr2[i] + i
        //arr1[i] + arr2[i] - i
        //arr1[i] - arr2[i] - i
        // 并计算max(arr) - min(arr)即可求得最终结果
        let mut max = [i32::MIN; 4];
        let mut min = [i32::MAX; 4];
        for i in 0..arr1.len() {
            max[0] = max[0].max(arr1[i] + arr2[i] + i as i32);
            min[0] = min[0].min(arr1[i] + arr2[i] + i as i32);
            max[1] = max[1].max(arr1[i] - arr2[i] + i as i32);
            min[1] = min[1].min(arr1[i] - arr2[i] + i as i32);
            max[2] = max[2].max(arr1[i] + arr2[i] - i as i32);
            min[2] = min[2].min(arr1[i] + arr2[i] - i as i32);
            max[3] = max[3].max(arr1[i] - arr2[i] - i as i32);
            min[3] = min[3].min(arr1[i] - arr2[i] - i as i32);
        }
        max.iter()
            .zip(min.iter())
            .into_iter()
            .map(|(i, j)| i - j)
            .max()
            .unwrap()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1131() {
        assert_eq!(
            Solution::max_abs_val_expr(vec![1, -2, -5, 0, 10], vec![0, -2, -1, -7, -4]),
            20
        );

        assert_eq!(
            Solution::max_abs_val_expr(vec![1, 2, 3, 4], vec![-1, 4, 5, 6]),
            13
        );

        assert_eq!(Solution::max_abs_val_expr(vec![1, -2], vec![8, 8]), 4);
    }
}
