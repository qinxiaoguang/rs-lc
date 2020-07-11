use super::ListNode;
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=82 lang=rust
 *
 * [82] 删除排序链表中的重复元素 II
 *
 * https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list-ii/description/
 *
 * algorithms
 * Medium (46.53%)
 * Likes:    302
 * Dislikes: 0
 * Total Accepted:    51.6K
 * Total Submissions: 107.3K
 * Testcase Example:  '[1,2,3,3,4,4,5]'
 *
 * 给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。
 *
 * 示例 1:
 *
 * 输入: 1->2->3->3->4->4->5
 * 输出: 1->2->5
 *
 *
 * 示例 2:
 *
 * 输入: 1->1->1->2->3
 * 输出: 2->3
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
use std::collections::HashMap;
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 只要有重复就删,注意是排序链表，所以其实就是相邻链表的删除
        // 暴力法就是维护一个map，如果数量大于1就删
        // 非暴力法还没想到
        let mut map = HashMap::new();
        let mut head = head;
        let mut cur = &mut head;
        while let Some(node) = cur {
            let mut cnt = map.entry(cur.as_ref()?.val).or_insert(0);
            *cnt += 1;
            cur = &mut cur.as_mut()?.next;
        }

        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut cur = &mut dummy_head;
        while let Some(node) = cur {
            // node.next如果不为空，且对应的map数据中的cnt个数大于0数，需要删掉
            if node.next.is_none() {
                break;
            }
            let val = &node.next.as_ref()?.val;
            if let Some(cnt) = map.get(val) {
                // 将next删掉
                if *cnt > 1 {
                    let remove_node = node.next.take();
                    if let Some(remove_node) = remove_node {
                        node.next = remove_node.next;
                    }
                    continue;
                }
            }
            cur = &mut cur.as_mut()?.next;
        }
        dummy_head?.next
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l082() {
        let list = ListNode::to_list(vec![1, 1, 1, 2, 3]);
        assert_eq!(
            ListNode::to_list(vec![2, 3]),
            Solution::delete_duplicates(list)
        );

        assert_eq!(ListNode::to_list(vec![]), Solution::delete_duplicates(None));
    }
}
