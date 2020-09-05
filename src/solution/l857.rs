pub struct Solution {}
/*
 * @lc app=leetcode.cn id=857 lang=rust
 *
 * [857] 雇佣 K 名工人的最低成本
 *
 * https://leetcode-cn.com/problems/minimum-cost-to-hire-k-workers/description/
 *
 * algorithms
 * Hard (44.98%)
 * Likes:    67
 * Dislikes: 0
 * Total Accepted:    1.6K
 * Total Submissions: 3.5K
 * Testcase Example:  '[10,20,5]\n[70,50,30]\n2'
 *
 * 有 N 名工人。 第 i 名工人的工作质量为 quality[i] ，其最低期望工资为 wage[i] 。
 *
 * 现在我们想雇佣 K 名工人组成一个工资组。在雇佣 一组 K 名工人时，我们必须按照下述规则向他们支付工资：
 *
 *
 * 对工资组中的每名工人，应当按其工作质量与同组其他工人的工作质量的比例来支付工资。
 * 工资组中的每名工人至少应当得到他们的最低期望工资。
 *
 *
 * 返回组成一个满足上述条件的工资组至少需要多少钱。
 *
 *
 *
 *
 *
 *
 * 示例 1：
 *
 * 输入： quality = [10,20,5], wage = [70,50,30], K = 2
 * 输出： 105.00000
 * 解释： 我们向 0 号工人支付 70，向 2 号工人支付 35。
 *
 * 示例 2：
 *
 * 输入： quality = [3,1,10,10,1], wage = [4,8,2,2,7], K = 3
 * 输出： 30.66667
 * 解释： 我们向 0 号工人支付 4，向 2 号和 3 号分别支付 13.33333。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= K <= N <= 10000，其中 N = quality.length = wage.length
 * 1 <= quality[i] <= 10000
 * 1 <= wage[i] <= 10000
 * 与正确答案误差在 10^-5 之内的答案将被视为正确的。
 *
 *
 */

// @lc code=start
use std::collections::BinaryHeap;
impl Solution {
    // 计算每个工人的性价比，性价比 quality/wage
    // 性价比越高的越便宜，但是如果再结算工资时，一定会按照性价最低的来结算。
    // 所以根据性价比来进行从大到小的排序
    // 假设选择了第i个工人的工资为基准，那么需要选前边k-1个 quality之和最小的,发的工资才最小
    // 使用大顶堆来维护前面的k-1个最小值，并遍历排序后的工人，以每个工人为基准时，计算出其最小工资数
    // 遍历一遍后即可计算出最小值。
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        if k == 0 {
            return 0f64;
        }
        let mut worker: Vec<(f64, f64)> = quality
            .into_iter()
            .map(|x| x as f64)
            .zip(wage.into_iter().map(|x| x as f64))
            .collect();
        worker.sort_by(|a, b| (b.0 / b.1).partial_cmp(&(a.0 / a.1)).unwrap());
        let mut heap = BinaryHeap::with_capacity(k as usize);
        let mut res = std::f64::MAX;
        for (q, a) in worker {
            if heap.len() == (k - 1) as usize {
                let sum = heap.iter().sum::<i32>();
                let tmp = (sum as f64 + q) * a / q;
                res = if res > tmp { tmp } else { res };
            }
            Self::push(&mut heap, q as i32, (k - 1) as usize);
        }
        res
    }

    fn push(heap: &mut BinaryHeap<i32>, value: i32, len: usize) {
        if len == 0 {
            return;
        }

        if heap.len() < len {
            heap.push(value);
            return;
        }
        if &value < heap.peek().unwrap() {
            heap.push(value);
            heap.pop();
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l857() {
        assert_eq!(
            105f64,
            Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2)
        );
        assert_eq!(
            30.666666666666668f64,
            Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3)
        );
        assert_eq!(2f64, Solution::mincost_to_hire_workers(vec![1], vec![2], 1));
    }
}
