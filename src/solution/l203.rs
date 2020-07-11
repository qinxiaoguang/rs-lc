use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=203 lang=rust
 *
 * [203] 移除链表元素
 *
 * https://leetcode-cn.com/problems/remove-linked-list-elements/description/
 *
 * algorithms
 * Easy (44.47%)
 * Likes:    405
 * Dislikes: 0
 * Total Accepted:    86.8K
 * Total Submissions: 189.7K
 * Testcase Example:  '[1,2,6,3,4,5,6]\n6'
 *
 * 删除链表中等于给定值 val 的所有节点。
 *
 * 示例:
 *
 * 输入: 1->2->6->3->4->5->6, val = 6
 * 输出: 1->2->3->4->5
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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut res_ref = res.as_mut();

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val != val {
                res_ref.as_mut()?.next = Some(node);
                res_ref = res_ref?.next.as_mut();
            }
        }
        res.as_mut()?.next.take()
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l203() {
        assert_eq!(
            ListNode::to_list(vec![4, 2, 1]),
            Solution::remove_elements(ListNode::to_list(vec![4, 2, 1, 3]), 3)
        );

        assert_eq!(
            None,
            Solution::remove_elements(ListNode::to_list(vec![]), 3)
        );
    }
}
