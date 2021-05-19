pub struct Solution {}
/*
 * @lc app=leetcode.cn id=932 lang=rust
 *
 * [932] 漂亮数组
 *
 * https://leetcode-cn.com/problems/beautiful-array/description/
 *
 * algorithms
 * Medium (59.70%)
 * Likes:    86
 * Dislikes: 0
 * Total Accepted:    4.8K
 * Total Submissions: 8K
 * Testcase Example:  '4'
 *
 * 对于某些固定的 N，如果数组 A 是整数 1, 2, ..., N 组成的排列，使得：
 *
 * 对于每个 i < j，都不存在 k 满足 i < k < j 使得 A[k] * 2 = A[i] + A[j]。
 *
 * 那么数组 A 是漂亮数组。
 *
 *
 *
 * 给定 N，返回任意漂亮数组 A（保证存在一个）。
 *
 *
 *
 * 示例 1：
 *
 * 输入：4
 * 输出：[2,1,4,3]
 *
 *
 * 示例 2：
 *
 * 输入：5
 * 输出：[3,1,2,5,4]
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= N <= 1000
 *
 *
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    // 注意是i<k<j,而不是i+1=k,k+1=j
    // 看了题解，这个题很没劲
    // 谁能想到这个方法
    // 对于 一个漂亮数组 [a1..an]，那么[ka1+b, ka2+b, ...kan +b]也是漂亮数组
    // 那么对于一个数组，[1~n]，可以先将其分为两部分，左部分全部为奇数，右部分全部为偶数
    // 这样的话左侧的任何两个数和右侧的任何一个数，或者左侧的任何一个数和右侧的任何两个数都不可能满足a[k]*2 = a[i]+a[j]
    // 但是怎么保证左侧任何三个数以及右侧任何三个数也不满足呢，答案就是分治
    // 当左右两部分分完之后，对左侧的奇数做 k=1/2, b = 1/2的仿射，右侧的偶数做k=1/2,b=0的仿射,这样得到的数组还是[1~n]，但是左右两部分的数就变成了奇偶混合的数
    // 就可以依然用上述的方法来进行分治
    // 但是真遇到这种问题，谁能想到这种方法呢?
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        Self::f(n, &mut m)
    }

    fn f(n: i32, m: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
        if m.contains_key(&n) {
            return m.get(&n).unwrap().clone();
        }

        let mut ans = vec![0; n as usize];
        if n == 1 {
            ans[0] = 1;
        } else {
            let mut t = 0;
            for x in Self::f((n + 1) / 2, m) {
                ans[t] = 2 * x - 1;
                t += 1;
            }
            for x in Self::f(n / 2, m) {
                ans[t] = 2 * x;
                t += 1;
            }
        }
        //println!("n:{},ans:{:?}", n, ans);
        m.insert(n, ans.clone());

        return ans;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l932() {
        println!("{:?}", Solution::beautiful_array(5));
    }
}
