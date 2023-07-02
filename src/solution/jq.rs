use super::Solution;
impl Solution {
    /*
    有一批货物, 我们必须按顺序在 d 天从一个地方运到另一个地方.
    货物重量使用数组 w 表示(数组长度小于 50000).
    每一天, 我们按顺序把货物放到车上运走, 货物重量不能超过车的最大承重量. 问: 车的最大承重量 最少是多少?

    eg:
    输入：w = [1,2,3,4,5,6,7,8,9,10], d = 5
    输出：15
    找到最小值和最大值，进行二分
     */
    fn min_weight(w: Vec<i32>, d: i32) -> i32 {
        if w.len() == 0 || d == 0 {
            return 0;
        }
        let check = |target| {
            // 贪心
            let mut tmp = 0;
            let mut day = 0;
            for x in w.iter() {
                if tmp + x > target {
                    day += 1;
                    tmp = *x;
                    if day > d {
                        return false;
                    }
                    continue;
                }
                tmp += x;
            }
            if tmp != 0 {
                day += 1;
            }
            return day <= d;
        };

        let mut left = w.iter().max().unwrap().clone();
        let mut right: i32 = w.iter().sum();
        // 二分找最小值
        while left < right {
            let mid = (left + right) / 2;
            if check(mid) {
                // 符合条件
                right = mid;
            } else {
                // 不符合条件
                left = mid + 1;
            }
        }

        return left;
    }

    // dp[n][m] 表示先手必赢
    // 则必存在 dp[n-1][m] .. dp[n-k][m]，dp[n][m-1],dp[n][m-k]为false
    // 那么反过来， 则若dp[n][m]为false,则dp[n+1][m]..dp[n+k][m],dp[n][m+1]..dp[n][m+k]为true
    // 首先当n==m的时候，先手必输，后手保证和先手取的数量一致即可取胜
    // 那么按此规则更新的dp有下图所示(假设k为3):
    // [ 0 1 1 1 x x ]
    // [ 1 0 1 1 1 x ]
    // [ 1 1 0 1 1 1 ]
    // [ 1 1 1 0 1 1 ]
    // 那么不确定的值就在右上角或左下角,这两个角完全没规律,难解。
    // 换个思路
    // 博弈
    // n==m的时候先手必输，那么n>m的时候，先手如果想赢就需要保证最后先手取的数使得n == m
    // 所以问题转换为 n-m的差值是 (k+1的倍数) + 1..k
    // 即如果符合该值，则先手可以先取 1..k任意一个，保证剩下的值都在k+1的倍数上，这样对方不论取1..k任意一值，先手只需要取对应的数保证和为k+1即可
    fn get_win_or_false(n: i32, m: i32, k: i32) -> bool {
        // 先手必输
        if n == m {
            return false;
        }

        let remain = (n - m).abs() % (k + 1);
        return remain >= 1 || remain <= k;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_jq() {
        assert_eq!(
            Solution::min_weight(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
        assert_eq!(Solution::min_weight(vec![], 5), 0);
        assert_eq!(Solution::min_weight(vec![1], 1), 1);
        assert_eq!(Solution::min_weight(vec![10], 1), 10);
        assert_eq!(Solution::min_weight(vec![1, 1, 1, 1, 1], 1), 5);
        assert_eq!(Solution::min_weight(vec![1, 1, 1, 1, 1], 2), 3);
    }

    #[test]
    fn test_jq2() {
        assert_eq!(Solution::get_win_or_false(1, 1, 2), false);
        assert_eq!(Solution::get_win_or_false(1, 2, 2), true);
    }
}
