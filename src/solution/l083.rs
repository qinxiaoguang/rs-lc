use super::ListNode;
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=83 lang=rust
 *
 * [83] 删除排序链表中的重复元素
 *
 * https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/description/
 *
 * algorithms
 * Easy (49.38%)
 * Likes:    333
 * Dislikes: 0
 * Total Accepted:    112.4K
 * Total Submissions: 221.9K
 * Testcase Example:  '[1,1,2]'
 *
 * 给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。
 *
 * 示例 1:
 *
 * 输入: 1->1->2
 * 输出: 1->2
 *
 *
 * 示例 2:
 *
 * 输入: 1->1->2->3->3
 * 输出: 1->2->3
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 与上一题不同的是，这个重复的元素要求保留一个，所以可以通过遍历的方式来解题
        let mut head = head;
        let mut cur = &mut head;
        while let Some(node) = cur {
            // 这里只能用node.next.take(),否则因为前边已经有了mut借用，导致后边无法引用或再借用
            // 而使用take则获取到所有权，而再使用完毕，再归还就行，也算是一个技巧吧。
            if let Some(mut next_node) = node.next.take() {
                if node.val == next_node.val {
                    node.next = next_node.next.take();
                    continue;
                } else {
                    node.next = Some(next_node);
                }
            }
            cur = &mut cur.as_mut()?.next;
        }
        head
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l083() {
        let list = ListNode::to_list(vec![1, 1, 1, 2, 3]);
        assert_eq!(
            ListNode::to_list(vec![1, 2, 3]),
            Solution::delete_duplicates(list)
        );

        assert_eq!(
            ListNode::to_list(vec![]),
            Solution::delete_duplicates(ListNode::to_list(vec![]))
        );

        assert_eq!(
            ListNode::to_list(vec![1, 2, 3]),
            Solution::delete_duplicates(ListNode::to_list(vec![1, 1, 2, 2, 3, 3]))
        );
    }
}
