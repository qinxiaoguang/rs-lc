use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=234 lang=rust
 *
 * [234] 回文链表
 *
 * https://leetcode-cn.com/problems/palindrome-linked-list/description/
 *
 * algorithms
 * Easy (40.84%)
 * Likes:    547
 * Dislikes: 0
 * Total Accepted:    101.8K
 * Total Submissions: 240.2K
 * Testcase Example:  '[1,2]'
 *
 * 请判断一个链表是否为回文链表。
 *
 * 示例 1:
 *
 * 输入: 1->2
 * 输出: false
 *
 * 示例 2:
 *
 * 输入: 1->2->2->1
 * 输出: true
 *
 *
 * 进阶：
 * 你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？
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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // 利用O(n)及O(1)空间复杂度解决
        // O(1)空间复杂度，表示无法使用栈递归
        // O(n)则表示只能遍历, 那么只能通过旋转后半部分的链表，与前部分再做比较了。
        // O(n)空间复杂度则是旋转链表，并逐个比较
        Self::reverse_list(head.clone()) == head
    }

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
    fn test_l234() {
        assert_eq!(
            true,
            Solution::is_palindrome(ListNode::to_list(vec![1, 2, 1]))
        );

        assert_eq!(
            false,
            Solution::is_palindrome(ListNode::to_list(vec![1, 2, 1, 1]))
        );

        assert_eq!(true, Solution::is_palindrome(ListNode::to_list(vec![])));

        assert_eq!(
            true,
            Solution::is_palindrome(ListNode::to_list(vec![1, 2, 2, 1]))
        );
    }
}
