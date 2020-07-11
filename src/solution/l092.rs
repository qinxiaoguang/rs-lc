use super::ListNode;
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=92 lang=rust
 *
 * [92] 反转链表 II
 *
 * https://leetcode-cn.com/problems/reverse-linked-list-ii/description/
 *
 * algorithms
 * Medium (49.38%)
 * Likes:    314
 * Dislikes: 0
 * Total Accepted:    38.3K
 * Total Submissions: 77.5K
 * Testcase Example:  '[1,2,3,4,5]\n2\n4'
 *
 * 反转从位置 m 到 n 的链表。请使用一趟扫描完成反转。
 *
 * 说明:
 * 1 ≤ m ≤ n ≤ 链表长度。
 *
 * 示例:
 *
 * 输入: 1->2->3->4->5->NULL, m = 2, n = 4
 * 输出: 1->4->3->2->5->NULL
 *
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        None
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l086() {
        assert_eq!(
            ListNode::to_list(vec![1, 2, 2, 4, 3, 5]),
            Solution::reverse_between(ListNode::to_list(vec![1, 4, 3, 2, 5, 2]), 1, 4)
        )
    }
}
