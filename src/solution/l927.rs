pub struct Solution {}
/*
 * @lc app=leetcode.cn id=927 lang=rust
 *
 * [927] 三等分
 *
 * https://leetcode-cn.com/problems/three-equal-parts/description/
 *
 * algorithms
 * Hard (30.34%)
 * Likes:    33
 * Dislikes: 0
 * Total Accepted:    1.8K
 * Total Submissions: 5.8K
 * Testcase Example:  '[1,0,1,0,1]'
 *
 * 给定一个由 0 和 1 组成的数组 A，将数组分成 3 个非空的部分，使得所有这些部分表示相同的二进制值。
 *
 * 如果可以做到，请返回任何 [i, j]，其中 i+1 < j，这样一来：
 *
 *
 * A[0], A[1], ..., A[i] 组成第一部分；
 * A[i+1], A[i+2], ..., A[j-1] 作为第二部分；
 * A[j], A[j+1], ..., A[A.length - 1] 是第三部分。
 * 这三个部分所表示的二进制值相等。
 *
 *
 * 如果无法做到，就返回 [-1, -1]。
 *
 * 注意，在考虑每个部分所表示的二进制时，应当将其看作一个整体。例如，[1,1,0] 表示十进制中的 6，而不会是 3。此外，前导零也是被允许的，所以
 * [0,1,1] 和 [1,1] 表示相同的值。
 *
 *
 *
 * 示例 1：
 *
 * 输入：[1,0,1,0,1]
 * 输出：[0,3]
 *
 *
 * 示例 2：
 *
 * 输出：[1,1,0,1,1]
 * 输出：[-1,-1]
 *
 *
 *
 * 提示：
 *
 *
 * 3 <= A.length <= 30000
 * A[i] == 0 或 A[i] == 1
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    //如果存在，则三个子数组的1的个数一定相等，如果1的总个数不能被3整除，则不存在
    // 找到平分3组1的位置
    // 但还需要注意，最后一组的后缀0，若有后缀0，则其他组的数据也必须要有后缀0
    pub fn three_equal_parts(a: Vec<i32>) -> Vec<i32> {
        let count: i32 = a.iter().sum(); // 1的个数
        let no_res = vec![-1, -1];
        println!("count:{}", count);
        if count % 3 != 0 {
            return no_res;
        }
        let avg_cnt = count / 3;
        println!("avg count:{}", avg_cnt);
        let (mut start, mut end) = (-1i32, -1i32);
        let mut tmp_cnt = 0;
        for idx in 0..a.len() {
            if a[idx] == 1 {
                tmp_cnt += 1;
            }
            if tmp_cnt == avg_cnt && start == -1 {
                start = idx as i32;
            }
            if tmp_cnt == avg_cnt * 2 && end == -1 {
                end = idx as i32 + 1;
            }
        }
        println!("start:{},end:{}", start, end);
        // 计算后缀0的个数
        let mut zero_cnt = 0i32;
        for idx in (0..a.len()).rev() {
            if a[idx] != 0 {
                break;
            }
            zero_cnt += 1;
        }
        if zero_cnt == a.len() as i32 {
            return vec![0, 2];
        }

        println!("zero_cnt:{}", zero_cnt);
        start += zero_cnt;
        end += zero_cnt;
        if end as usize >= a.len() {
            return no_res;
        }
        println!("after start:{},end:{}", start, end);

        // 判断a[0..=start] a[start+1..=end] a[end..]是否相等
        let s1 = a[0..=start as usize]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        let s2 = a[start as usize + 1..end as usize]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        let s3 = a[end as usize..]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        println!("before:s1:{}, s2:{}, s3:{}", s1, s2, s3);
        let s1 = s1.trim_start_matches(|x| x == '0');
        let s2 = s2.trim_start_matches(|x| x == '0');
        let s3 = s3.trim_start_matches(|x| x == '0');
        println!("after:s1:{}, s2:{}, s3:{}", s1, s2, s3);

        if s1 == s2 && s2 == s3 {
            return vec![start as i32, end as i32];
        }
        return no_res;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l927() {
        assert_eq!(vec![0, 3], Solution::three_equal_parts(vec![1, 0, 1, 0, 1]));
        assert_eq!(
            vec![-1, -1],
            Solution::three_equal_parts(vec![1, 1, 0, 1, 1])
        );
        assert_eq!(vec![0, 2], Solution::three_equal_parts(vec![0, 0, 0, 0, 0]));
    }
}
