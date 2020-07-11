use super::ListNode;
pub struct Solution {}
/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
 *
 * https://leetcode-cn.com/problems/reverse-nodes-in-k-group/description/
 *
 * algorithms
 * Hard (56.85%)
 * Likes:    614
 * Dislikes: 0
 * Total Accepted:    77.3K
 * Total Submissions: 125.6K
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
 *
 * k 是一个正整数，它的值小于或等于链表的长度。
 *
 * 如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
 *
 *
 *
 * 示例：
 *
 * 给你这个链表：1->2->3->4->5
 *
 * 当 k = 2 时，应当返回: 2->1->4->3->5
 *
 * 当 k = 3 时，应当返回: 3->2->1->4->5
 *
 *
 *
 * 说明：
 *
 *
 * 你的算法只能使用常数的额外空间。
 * 你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。
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
    // 抄自，实在不知道怎么写了。https://github.com/aylei/leetcode-rust
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = dummy_head.as_mut();
        'outer: loop {
            let mut start = head.as_mut()?.next.take();
            if start.is_none() {
                break;
            }
            let mut end = start.as_mut();
            for _ in 0..(k - 1) {
                end = end?.next.as_mut();
                if end.is_none() {
                    head.as_mut()?.next = start;
                    break 'outer;
                }
            }
            let mut tail = end.as_mut()?.next.take();
            let end = Self::reverse(start, tail);
            head.as_mut()?.next = end;
            for _ in 0..k {
                head = head?.next.as_mut();
            }
        }
        dummy_head?.next
    }

    #[inline(always)]
    fn reverse(
        mut head: Option<Box<ListNode>>,
        tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev = tail;
        let mut current = head;
        while let Some(mut current_node_inner) = current {
            let mut next = current_node_inner.next.take();
            current_node_inner.next = prev.take();
            prev = Some(current_node_inner);
            current = next;
        }
        prev
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l025() {
        let list = ListNode::to_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(
            ListNode::to_list(vec![3, 2, 1, 4, 5]),
            Solution::reverse_k_group(list, 3)
        );
    }
}
