use super::Solution;

impl Solution {
    // 遍历一遍，按照公式 (start + x)*(x-start+1)/2 = target
    // 可以得 x(x+1) = 2*target+ s^2 - s
    // 是个关于x的一元2次方程，ax^2 +bx +c = 0的解为x = [-b +/- (b^2-4ac).sqrt() ]/2a
    // 只要解是个整数就行
    // 该题中a=1, b=1, c= s-s^2-2*target
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        (1..=target as i64).for_each(|n| {
            let c = -(2 * target as i64 + n * n - n);
            let sqrt = ((1 - 4 * c) as f64).sqrt(); // 该数必须是整数
            if !Self::is_i32(sqrt) {
                return;
            }
            let x1 = (-1f64 + sqrt) / (2f64);
            if Self::is_i32(x1) && (x1 as i64 - n) >= 1 {
                res.push((n as i32..=x1 as i32).into_iter().collect());
                return;
            }
            let x1 = (-1f64 - sqrt) / (2f64);
            if Self::is_i32(x1) && (x1 as i64 - n) >= 1 {
                res.push((n as i32..=x1 as i32).into_iter().collect());
            }
        });
        res
    }

    fn is_i32(f: f64) -> bool {
        return f.floor() == f;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j057v2() {
        assert_eq!(
            Solution::find_continuous_sequence(9),
            matrix![[2, 3, 4], [4, 5]]
        );
        assert_eq!(
            Solution::find_continuous_sequence(15),
            matrix![[1, 2, 3, 4, 5], [4, 5, 6], [7, 8]]
        );
        let res = Solution::find_continuous_sequence(98160);
    }
}
