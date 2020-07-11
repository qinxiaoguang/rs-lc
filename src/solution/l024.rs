use super::ListNode;
pub struct Solution {}
/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 *
 * https://leetcode-cn.com/problems/swap-nodes-in-pairs/description/
 *
 * algorithms
 * Medium (64.74%)
 * Likes:    535
 * Dislikes: 0
 * Total Accepted:    118.1K
 * Total Submissions: 179K
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。
 *
 * 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
 *
 *
 *
 * 示例:
 *
 * 给定 1->2->3->4, 你应该返回 2->1->4->3.
 *
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 递归解决
        let mut head = head;
        if let Some(mut head_node) = head.as_mut() {
            let mut sec = head_node.next.take();
            if let Some(mut sec_node) = sec.as_mut() {
                let three = sec_node.next.take();
                let swap_node = Self::swap_pairs(three);
                head_node.next = swap_node;
                sec_node.next = head;
                return sec;
            }
        }
        return head;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l024() {
        assert_eq!(
            ListNode::to_list(vec![2, 1, 3]),
            Solution::swap_pairs(ListNode::to_list(vec![1, 2, 3]))
        );
        assert_eq!(
            ListNode::to_list(vec![2, 1, 4, 3]),
            Solution::swap_pairs(ListNode::to_list(vec![1, 2, 3, 4]))
        );

        assert_eq!(
            ListNode::to_list(vec![1]),
            Solution::swap_pairs(ListNode::to_list(vec![1]))
        );

        assert_eq!(None, Solution::swap_pairs(ListNode::to_list(vec![])));
    }
}
