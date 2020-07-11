use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=148 lang=rust
 *
 * [148] 排序链表
 *
 * https://leetcode-cn.com/problems/sort-list/description/
 *
 * algorithms
 * Medium (64.14%)
 * Likes:    612
 * Dislikes: 0
 * Total Accepted:    71.9K
 * Total Submissions: 108.9K
 * Testcase Example:  '[4,2,1,3]'
 *
 * 在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序。
 *
 * 示例 1:
 *
 * 输入: 4->2->1->3
 * 输出: 1->2->3->4
 *
 *
 * 示例 2:
 *
 * 输入: -1->5->3->4->0
 * 输出: -1->0->3->4->5
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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // O(nlogn)对链表排序
        // 既然是 nlogn，就可以使用分治的思想来进行排序
        // 首先选取第一个结点作为临界点，将小于该点的结点放在第一个链表里，大于该点的结点放在第二个链表里
        // 看官网上，有人先把链表转成vec,直接通过vec.sort_unstable()来排序，有点牛披
        let (mut small_link, mut big_link) = (
            Some(Box::new(ListNode::new(0))),
            Some(Box::new(ListNode::new(0))),
        );
        let (mut small_ref, mut big_ref) = (small_link.as_mut(), big_link.as_mut());
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mid_val = head.as_ref()?.val;
        let mut remain = head.as_mut()?.next.take();
        while let Some(mut node) = remain {
            remain = node.next.take();
            if node.val < mid_val {
                small_ref.as_mut()?.next = Some(node);
                small_ref = small_ref?.next.as_mut();
            } else {
                big_ref.as_mut()?.next = Some(node);
                big_ref = big_ref?.next.as_mut();
            }
        }

        // 分治思想，对左右链表分别进行排序
        let mut left_link = Self::sort_list(small_link?.next.take());
        let mut right_link = Self::sort_list(big_link?.next.take());
        // 最后合并
        let mut res_head = Some(Box::new(ListNode::new(0)));
        let mut res_ref = res_head.as_mut();
        if left_link.is_some() {
            res_ref.as_mut()?.next = left_link;
        }
        while let Some(_) = res_ref.as_mut()?.next.as_mut() {
            res_ref = res_ref?.next.as_mut();
        }
        res_ref.as_mut()?.next = Some(Box::new(ListNode::new(mid_val)));
        res_ref = res_ref?.next.as_mut();

        if right_link.is_some() {
            res_ref.as_mut()?.next = right_link;
        }

        res_head?.next.take()
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l148() {
        assert_eq!(
            ListNode::to_list(vec![1, 2, 3, 4]),
            Solution::sort_list(ListNode::to_list(vec![4, 2, 1, 3]))
        );

        assert_eq!(
            ListNode::to_list(vec![-1, 0, 3, 4, 5]),
            Solution::sort_list(ListNode::to_list(vec![-1, 5, 3, 4, 0]))
        );

        assert_eq!(None, Solution::sort_list(ListNode::to_list(vec![])));
        assert_eq!(
            ListNode::to_list(vec![1]),
            Solution::sort_list(ListNode::to_list(vec![1]))
        );
    }
}
