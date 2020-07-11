use super::ListNode;
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 *
 * https://leetcode-cn.com/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (60.26%)
 * Likes:    1110
 * Dislikes: 0
 * Total Accepted:    292.2K
 * Total Submissions: 464.6K
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
 *
 *
 *
 * 示例：
 *
 * 输入：1->2->4, 1->3->4
 * 输出：1->1->2->3->4->4
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

// 单独的ListNode构成的链表无法实现Pop方法
// 因为Self无法pop完self后，继续使用self
//
// 需要一个头结点，头结点不包含任何值，只指向第一个ListNode
// 这样的话就可以通过头结点，来将头结点指向的第一个结点pop出来
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut lhs, mut rhs) = (l1, l2);
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;

        while let (Some(lnode), Some(rnode)) = (lhs.as_ref(), rhs.as_ref()) {
            if lnode.val <= rnode.val {
                tail.next = lhs;
                tail = tail.next.as_mut().unwrap();
                lhs = tail.next.take();
            } else {
                tail.next = rhs;
                tail = tail.next.as_mut().unwrap();
                rhs = tail.next.take();
            }
        }
        tail.next = if lhs.is_some() { lhs } else { rhs };
        head.next
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l021() {
        let mut l1 = ListNode::new(1);
        let mut l1_2 = ListNode::new(2);
        let l1_3 = ListNode::new(3);
        l1_2.next = Some(Box::new(l1_3));
        l1.next = Some(Box::new(l1_2));

        let mut l2 = ListNode::new(3);
        let mut l2_2 = ListNode::new(4);
        let l2_3 = ListNode::new(5);
        l2_2.next = Some(Box::new(l2_3));
        l2.next = Some(Box::new(l2_2));

        let res = Solution::merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));
        println!("{:?}", res);
    }
}
