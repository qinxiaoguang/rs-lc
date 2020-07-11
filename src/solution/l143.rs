use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=143 lang=rust
 *
 * [143] 重排链表
 *
 * https://leetcode-cn.com/problems/reorder-list/description/
 *
 * algorithms
 * Medium (54.97%)
 * Likes:    245
 * Dislikes: 0
 * Total Accepted:    29.4K
 * Total Submissions: 52.5K
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给定一个单链表 L：L0→L1→…→Ln-1→Ln ，
 * 将其重新排列后变为： L0→Ln→L1→Ln-1→L2→Ln-2→…
 *
 * 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
 *
 * 示例 1:
 *
 * 给定链表 1->2->3->4, 重新排列为 1->4->2->3.
 *
 * 示例 2:
 *
 * 给定链表 1->2->3->4->5, 重新排列为 1->5->2->4->3.
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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        // 1. 获取链表长度
        let mut len = 0;
        let mut cur = head.as_mut().unwrap().next.as_mut();
        while let Some(node) = cur {
            cur = node.next.as_mut();
            len += 1;
        }

        // 2. 从中间截断链表
        let mut tmp_len = 0;
        let mut cur = head.as_mut().unwrap().next.as_mut();
        let mut mid = None;

        while let Some(node) = cur {
            if tmp_len == len / 2 - 1 {
                mid = node.next.take();
                break;
            }
            cur = node.next.as_mut();
            tmp_len += 1;
        }

        // 3. 将后半断链表进行反转
        mid = Self::reverse_list(mid);

        // 4. 将两个链表合并
        let mut cur = head.as_mut();
        while let (Some(first), Some(mut sec)) = (cur, mid) {
            let mut tmp = first.next.take();
            let sec_tmp = sec.next.take();
            sec.next = tmp;
            first.next = Some(sec);
            mid = sec_tmp;
            cur = first.next.as_mut().unwrap().next.as_mut();
        }
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
    fn test_l143() {
        let mut list = ListNode::to_list(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut list);
        assert_eq!(ListNode::to_list(vec![1, 4, 2, 3]), list);

        let mut none_list = ListNode::to_list(vec![]);
        Solution::reorder_list(&mut none_list);
        assert_eq!(None, none_list);

        let mut list = ListNode::to_list(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut list);
        assert_eq!(ListNode::to_list(vec![1, 5, 2, 4, 3]), list);

        let mut list = ListNode::to_list(vec![1]);
        Solution::reorder_list(&mut list);
        assert_eq!(ListNode::to_list(vec![1]), list);

        let mut list = ListNode::to_list(vec![1, 2]);
        Solution::reorder_list(&mut list);
        assert_eq!(ListNode::to_list(vec![1, 2]), list);

        let mut list = ListNode::to_list(vec![1, 2, 3]);
        Solution::reorder_list(&mut list);
        assert_eq!(ListNode::to_list(vec![1, 3, 2]), list);

        let mut list = ListNode::to_list(vec![1, 2, 3, 4, 5, 6]);
        Solution::reorder_list(&mut list);
        assert_eq!(ListNode::to_list(vec![1, 6, 2, 5, 3, 4]), list);
    }
}
