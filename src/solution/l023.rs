use super::ListNode;
pub struct Solution {}
/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并K个排序链表
 *
 * https://leetcode-cn.com/problems/merge-k-sorted-lists/description/
 *
 * algorithms
 * Hard (49.33%)
 * Likes:    756
 * Dislikes: 0
 * Total Accepted:    136K
 * Total Submissions: 261.1K
 * Testcase Example:  '[[1,4,5],[1,3,4],[2,6]]'
 *
 * 合并 k 个排序链表，返回合并后的排序链表。请分析和描述算法的复杂度。
 *
 * 示例:
 *
 * 输入:
 * [
 * 1->4->5,
 * 1->3->4,
 * 2->6
 * ]
 * 输出: 1->1->2->3->4->4->5->6
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        lists.into_iter().fold(None, Self::merge_two_lists)
    }

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
    fn test_l023() {
        let v = vec![1, 2, 3];
        let l1 = ListNode::to_list(v);
        let v = vec![3, 4, 5];
        let l2 = ListNode::to_list(v);
        let v = vec![0, 7, 9];
        let l3 = ListNode::to_list(v);

        let res = Solution::merge_k_lists(vec![l1, l2, l3]);
        println!("{:?}", res);
    }
}
