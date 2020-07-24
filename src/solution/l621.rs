pub struct Solution {}

/*
 * @lc app=leetcode.cn id=621 lang=rust
 *
 * [621] 任务调度器
 *
 * https://leetcode-cn.com/problems/task-scheduler/description/
 *
 * algorithms
 * Medium (48.06%)
 * Likes:    317
 * Dislikes: 0
 * Total Accepted:    27K
 * Total Submissions: 53.9K
 * Testcase Example:  '["A","A","A","B","B","B"]\n2'
 *
 * 给定一个用字符数组表示的 CPU 需要执行的任务列表。其中包含使用大写的 A - Z 字母表示的26
 * 种不同种类的任务。任务可以以任意顺序执行，并且每个任务都可以在 1 个单位时间内执行完。CPU
 * 在任何一个单位时间内都可以执行一个任务，或者在待命状态。
 *
 * 然而，两个相同种类的任务之间必须有长度为 n 的冷却时间，因此至少有连续 n 个单位时间内 CPU 在执行不同的任务，或者在待命状态。
 *
 * 你需要计算完成所有任务所需要的最短时间。
 *
 *
 *
 * 示例 ：
 *
 * 输入：tasks = ["A","A","A","B","B","B"], n = 2
 * 输出：8
 * 解释：A -> B -> (待命) -> A -> B -> (待命) -> A -> B.
 * ⁠    在本示例中，两个相同类型任务之间必须间隔长度为 n = 2
 * 的冷却时间，而执行一个任务只需要一个单位时间，所以中间出现了（待命）状态。
 *
 *
 *
 * 提示：
 *
 *
 * 任务的总个数为 [1, 10000]。
 * n 的取值范围为 [0, 100]。
 *
 *
 */

// @lc code=start
impl Solution {
    // 最佳结果就是任务个数排序，一轮一轮的完成，每轮完成，都会增加下个任务的间隔时间
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut cnt_map = [0; 26];
        for c in tasks {
            cnt_map[(c as usize - 'A' as usize)] += 1;
        }
        let mut v = vec![];
        for (c, cnt) in cnt_map.iter().enumerate() {
            v.push((c, cnt));
        }
        v.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        0
        // TODO 太长 不想写了
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l621() {}
}
