pub struct Solution {}
/*
 * @lc app=leetcode.cn id=798 lang=rust
 *
 * [798] 得分最高的最小轮调
 *
 * https://leetcode-cn.com/problems/smallest-rotation-with-highest-score/description/
 *
 * algorithms
 * Hard (41.71%)
 * Likes:    44
 * Dislikes: 0
 * Total Accepted:    699
 * Total Submissions: 1.7K
 * Testcase Example:  '[2,3,1,4,0]'
 *
 * 给定一个数组 A，我们可以将它按一个非负整数 K 进行轮调，这样可以使数组变为 A[K], A[K+1], A{K+2], ... A[A.length
 * - 1], A[0], A[1], ..., A[K-1] 的形式。此后，任何值小于或等于其索引的项都可以记作一分。
 *
 * 例如，如果数组为 [2, 4, 1, 3, 0]，我们按 K = 2 进行轮调后，它将变成 [1, 3, 0, 2, 4]。这将记作 3 分，因为 1
 * > 0 [no points], 3 > 1 [no points], 0 <= 2 [one point], 2 <= 3 [one point],
 * 4 <= 4 [one point]。
 *
 * 在所有可能的轮调中，返回我们所能得到的最高分数对应的轮调索引 K。如果有多个答案，返回满足条件的最小的索引 K。
 *
 *
 *
 * 示例 1：
 *
 * 输入：[2, 3, 1, 4, 0]
 * 输出：3
 * 解释：
 * 下面列出了每个 K 的得分：
 * K = 0,  A = [2,3,1,4,0],    score 2
 * K = 1,  A = [3,1,4,0,2],    score 3
 * K = 2,  A = [1,4,0,2,3],    score 3
 * K = 3,  A = [4,0,2,3,1],    score 4
 * K = 4,  A = [0,2,3,1,4],    score 3
 * 所以我们应当选择 K = 3，得分最高。
 *
 * 示例 2：
 *
 * 输入：[1, 3, 0, 2, 4]
 * 输出：0
 * 解释：
 * A 无论怎么变化总是有 3 分。
 * 所以我们将选择最小的 K，即 0。
 *
 *
 *
 *
 * 提示：
 *
 *
 * A 的长度最大为 20000。
 * A[i] 的取值范围是 [0, A.length]。
 *
 *
 */

// @lc code=start
impl Solution {
    // 其实就是求数组中的数小于其下标的最大个数
    // 但是我们不需要傻到真的去对数组进行轮转
    // 此题是hard,所以一般不会采用暴力去解
    // 假设有一个数 A[i] = num,他在k=[l,r]的区间内是不会得分
    // 注意k值其实就是左旋的个数
    // 因为k=[l,r)时，A[i]这个数移动到了[0~num]中的某个位置，这个位置是不会得分的
    // 那么求出l,r是比较重要的，可以找到这么一个规律，一般而言 l = (i-A[i]+1+N)%N
    // r = i + 1, (r-l = A[i]),r表示的是移动到-1的位置时k的值，该位置是该数刚好可以得分的位置
    // 而l则是刚好不得分的位置
    // 如果有这么一个数组 bad,bad[i]表示数组中所有数在第i次论调后，刚好不得分的个数
    // 注意一点 当k移动的时候，下标会减1，所以部分不得分的数还是会不得分。
    // 那么对于 k而言，其得分数则为 bad[0~k]的求和
    // 所以如何求bad[n]是关键，看官方题解看得蒙蔽，想了半天才有一点头绪
    pub fn best_rotation(a: Vec<i32>) -> i32 {
        let N = a.len();
        let mut bad = vec![0; N];
        for i in 0..N {
            let left = (i as i32 - a[i] + 1 + N as i32) % N as i32;
            let right = (i as i32 + 1) % N as i32;
            bad[left as usize] -= 1;
            bad[right as usize] += 1;
            if left > right {
                bad[0] -= 1;
            }
        }
        let mut best = -(N as i32);
        let mut ans = 0;
        let mut cur = 0;
        for i in 0..N {
            cur += bad[i];
            if cur > best {
                best = cur;
                ans = i;
            }
        }
        return ans as i32;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l798() {}
}
