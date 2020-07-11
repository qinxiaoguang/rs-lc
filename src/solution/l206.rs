use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 *
 * https://leetcode-cn.com/problems/reverse-linked-list/description/
 *
 * algorithms
 * Easy (67.98%)
 * Likes:    1067
 * Dislikes: 0
 * Total Accepted:    272.5K
 * Total Submissions: 391.1K
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * 反转一个单链表。
 *
 * 示例:
 *
 * 输入: 1->2->3->4->5->NULL
 * 输出: 5->4->3->2->1->NULL
 *
 * 进阶:
 * 你可以迭代或递归地反转链表。你能否用两种方法解决这道题？
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = None;
        let mut head = head;
        while let Some(mut node) = head {
            let next = node.next.take();
            node.next = res;
            head = next;
            res = Some(node)
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l206() {
        assert_eq!(
            ListNode::to_list(vec![1, 2, 3]),
            Solution::reverse_list(ListNode::to_list(vec![3, 2, 1]))
        );

        assert_eq!(None, Solution::reverse_list(ListNode::to_list(vec![])));
    }
}
