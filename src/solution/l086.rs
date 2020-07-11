use super::ListNode;
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=86 lang=rust
 *
 * [86] 分隔链表
 *
 * https://leetcode-cn.com/problems/partition-list/description/
 *
 * algorithms
 * Medium (56.42%)
 * Likes:    219
 * Dislikes: 0
 * Total Accepted:    41.2K
 * Total Submissions: 70.6K
 * Testcase Example:  '[1,4,3,2,5,2]\n3'
 *
 * 给定一个链表和一个特定值 x，对链表进行分隔，使得所有小于 x 的节点都在大于或等于 x 的节点之前。
 *
 * 你应当保留两个分区中每个节点的初始相对位置。
 *
 * 示例:
 *
 * 输入: head = 1->4->3->2->5->2, x = 3
 * 输出: 1->2->2->4->3->5
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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        // 生成两个链表，一个小于x的链表，一个大于x的链表，最后把这两个链表连接即可。
        // dsh = dummy_small_head
        // dbh = dummy_big_head
        let (mut dsh, mut dbh) = (
            Some(Box::new(ListNode::new(0))),
            Some(Box::new(ListNode::new(0))),
        );
        let (mut dsh_mut, mut dbh_mut) = (dsh.as_mut(), dbh.as_mut());

        let mut cur = head;

        while let Some(mut node) = cur {
            let next = node.next.take();
            if node.val < x {
                dsh_mut.as_mut()?.next = Some(node);
                dsh_mut = dsh_mut?.next.as_mut();
            } else {
                dbh_mut.as_mut()?.next = Some(node);
                dbh_mut = dbh_mut?.next.as_mut();
            }
            // cur = next 实际上就是cur = node.next.take()
            // 如果在退出的时候cur被move了，在下次循环的时候因为又要用到cur
            // 此时就会报错，正确的方法应该当是，每次在下次循环前保证cur有效
            cur = next;
        }
        dsh_mut.as_mut()?.next = dbh?.next.take();
        dsh?.next
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l086() {
        assert_eq!(
            ListNode::to_list(vec![1, 2, 2, 4, 3, 5]),
            Solution::partition(ListNode::to_list(vec![1, 4, 3, 2, 5, 2]), 3)
        )
    }
}
