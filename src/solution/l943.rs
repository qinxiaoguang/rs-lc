pub struct Solution {}
/*
 * @lc app=leetcode.cn id=943 lang=rust
 *
 * [943] 最短超级串
 *
 * https://leetcode-cn.com/problems/find-the-shortest-superstring/description/
 *
 * algorithms
 * Hard (44.04%)
 * Likes:    48
 * Dislikes: 0
 * Total Accepted:    1K
 * Total Submissions: 2.3K
 * Testcase Example:  '["alex","loves","leetcode"]'
 *
 * 给定一个字符串数组 A，找到以 A 中每个字符串作为子字符串的最短字符串。
 *
 * 我们可以假设 A 中没有字符串是 A 中另一个字符串的子字符串。
 *
 *
 *
 * 示例 1：
 *
 * 输入：["alex","loves","leetcode"]
 * 输出："alexlovesleetcode"
 * 解释："alex"，"loves"，"leetcode" 的所有排列都会被接受。
 *
 * 示例 2：
 *
 * 输入：["catg","ctaagt","gcta","ttca","atgcatc"]
 * 输出："gctaagttcatgcatc"
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= A.length <= 12
 * 1 <= A[i].length <= 20
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    // 设l[i][j]为第i个字符串的后缀与第j个字符的前缀的公共最长串的长度
    // 则l[j][i]与l[i][j]并不相等。且计算l没有最优方法
    // 则该题变为求一个序列 a1~an,使得 l[a1][a2] + l[a2][a3] + .. l[an-1][an]最大
    // 可以通过dp来完成，dp[i][j]表示当前已选了i个字符串，且以a[j]结尾的最短长度
    // 为了让dp保存当前已选的都有哪些字符串，使用mask来代表i, 其中mask是二进制的数据，哪个位上的数据被使用了，则该位的数为1
    // 那么dp[i][j] = min ( dp[i-1][1~n]+ l[1~n][1~n])
    // 代码太难写了，写一天都不一定能写出来。放弃
    pub fn shortest_superstring(a: Vec<String>) -> String {
        "".to_string()
    }

    // 计算a的尾与b的前缀的最长公共长度
    fn get_len(a: &str, b: &str) -> i32 {
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let min_len = if a.len() > b.len() { b.len() } else { a.len() };
        let mut res = 0;
        for i in (0..min_len).rev() {
            if a.ends_with(&b[..i]) {
                return i as i32;
            }
        }
        return 0;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l943() {
        Solution::shortest_superstring(vec![
            String::from("catg"),
            String::from("ctaagt"),
            String::from("gcta"),
            String::from("ttca"),
            String::from("atgcatc"),
        ]);
    }
}
