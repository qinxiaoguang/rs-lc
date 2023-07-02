use super::*;

impl Solution {
    // 狒狒吃香蕉，求最小速度
    // 本身是枚举，因为有单调递增的规律，所以使用二分
    // 目标k，比k大的肯定都能在h小时内吃完，而比k小的，肯定不能在h小时内吃完,按这个方式二分就可以
    // 而k的最小值是1,最大值是数组中的max
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut l, mut r) = (1, *piles.iter().max().unwrap());

        while l < r {
            let mid = (l + r) / 2;

            if piles.iter().map(|x| (x - 1) / mid + 1).sum::<i32>() > h {
                // 不能在h小时内吃完
                l = mid + 1;
                continue;
            }
            // 可以在h小时内吃完
            r = mid;
        }
        return l;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_073() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }
}
