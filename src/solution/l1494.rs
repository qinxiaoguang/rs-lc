use super::Solution;

impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        // 状态压缩，就是将状态压缩到一个整数里，采用二进制或其他更简单的方式来表示状态，这样就能针对集合类型做dp
        // 比如该题，dp[i]表示i这个集合，在学完时，至少需要多少学期
        // 那么知道dp[i]后，可以将剩余的集合进行枚举，再进行更新，枚举方法
        // 步进采用 (mask-1)&state进行枚举,初始化mast=state
        // 这种状态枚举的方法要死记
        let mut dp = vec![-1; 1 << (n + 1) as usize];
        Self::l1494_dfs(0, &mut dp, n, k, &relations)
    }

    fn l1494_dfs(mask: usize, dp: &mut Vec<i32>, n: i32, k: i32, relations: &Vec<Vec<i32>>) -> i32 {
        if dp[mask] != -1 {
            dp[mask]
        } else if mask == ((1 << n) - 1) {
            0
        } else {
            // 初始状态
            let mut state = ((1 << n) - 1);
            // 查找当前mask下，能选的集合
            for i in relations {
                let (u, v) = (i[0] - 1, i[1] - 1);
                // u-> v 都没用过，v不可选，将v排除
                if ((mask >> u) & 1) == 0 && ((mask >> v) & 1) == 0 {
                    state &= !(1 << v);
                }
            }
            // mask中剩余的没用过的，和符合条件的state求并集
            state &= !mask;
            // 对state进行枚举
            let mut ans = i32::MAX;
            let mut next_mask = state;
            while next_mask != 0 {
                if next_mask.count_ones() <= k as u32 {
                    // mask | next_mask表示next_mask被接受了
                    ans = ans.min(Self::l1494_dfs(mask | next_mask, dp, n, k, relations) + 1);
                }
                next_mask = (next_mask - 1) & state;
            }
            dp[mask] = ans;
            ans
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1494() {
        // n = 5, dependencies = [[2,1],[3,1],[4,1],[1,5]], k = 2
        assert_eq!(
            Solution::min_number_of_semesters(
                5,
                vec![vec![2, 1], vec![3, 1], vec![4, 1], vec![1, 5]],
                2
            ),
            4
        );
    }
}
