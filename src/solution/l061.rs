use super::ListNode;
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=61 lang=rust
 *
 * [61] 旋转链表
 *
 * https://leetcode-cn.com/problems/rotate-list/description/
 *
 * algorithms
 * Medium (39.98%)
 * Likes:    281
 * Dislikes: 0
 * Total Accepted:    68.8K
 * Total Submissions: 170.2K
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给定一个链表，旋转链表，将链表每个节点向右移动 k 个位置，其中 k 是非负数。
 *
 * 示例 1:
 *
 * 输入: 1->2->3->4->5->NULL, k = 2
 * 输出: 4->5->1->2->3->NULL
 * 解释:
 * 向右旋转 1 步: 5->1->2->3->4->NULL
 * 向右旋转 2 步: 4->5->1->2->3->NULL
 *
 *
 * 示例 2:
 *
 * 输入: 0->1->2->NULL, k = 4
 * 输出: 2->0->1->NULL
 * 解释:
 * 向右旋转 1 步: 2->0->1->NULL
 * 向右旋转 2 步: 1->2->0->NULL
 * 向右旋转 3 步: 0->1->2->NULL
 * 向右旋转 4 步: 2->0->1->NULL
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 边界处理
        if head.is_none() {
            return head;
        }

        // 计算链表长度
        let mut head = head;
        let mut len = 0;
        let mut cur = &mut head;
        while let Some(_) = cur {
            len += 1;
            cur = &mut cur.as_mut()?.next;
        }

        // 计算右转的长度
        let next_cnt = len - k % len;
        let mut new_dummy_head = &mut head;
        if next_cnt == len {
            return head;
        }

        // 找到右转后的头结点
        for _ in 0..(next_cnt - 1) {
            new_dummy_head = &mut new_dummy_head.as_mut()?.next;
        }
        let mut new_head = new_dummy_head.as_mut()?.next.take();
        let mut cur = &mut new_head;

        // 找到尾结点，并将尾结点连接到头结点
        loop {
            if cur.as_mut()?.next.is_none() {
                break;
            }
            cur = &mut cur.as_mut()?.next;
        }

        cur.as_mut()?.next = head;
        new_head
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l061() {
        let list = ListNode::to_list(vec![1, 2, 3]);
        assert_eq!(
            ListNode::to_list(vec![3, 1, 2]),
            Solution::rotate_right(list.clone(), 1)
        );

        assert_eq!(
            ListNode::to_list(vec![2, 3, 1]),
            Solution::rotate_right(list.clone(), 2)
        );

        assert_eq!(
            ListNode::to_list(vec![1, 2, 3]),
            Solution::rotate_right(list.clone(), 3)
        );

        assert_eq!(
            ListNode::to_list(vec![3, 1, 2]),
            Solution::rotate_right(list.clone(), 4)
        );

        assert_eq!(ListNode::to_list(vec![]), Solution::rotate_right(None, 0));
    }
}
