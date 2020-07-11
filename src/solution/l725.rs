use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=725 lang=rust
 *
 * [725] 分隔链表
 *
 * https://leetcode-cn.com/problems/split-linked-list-in-parts/description/
 *
 * algorithms
 * Medium (53.29%)
 * Likes:    79
 * Dislikes: 0
 * Total Accepted:    10.5K
 * Total Submissions: 19.2K
 * Testcase Example:  '[1,2,3,4]\n5'
 *
 * 给定一个头结点为 root 的链表, 编写一个函数以将链表分隔为 k 个连续的部分。
 *
 * 每部分的长度应该尽可能的相等: 任意两部分的长度差距不能超过 1，也就是说可能有些部分为 null。
 *
 * 这k个部分应该按照在链表中出现的顺序进行输出，并且排在前面的部分的长度应该大于或等于后面的长度。
 *
 * 返回一个符合上述规则的链表的列表。
 *
 * 举例： 1->2->3->4, k = 5 // 5 结果 [ [1], [2], [3], [4], null ]
 *
 * 示例 1：
 *
 *
 * 输入:
 * root = [1, 2, 3], k = 5
 * 输出: [[1],[2],[3],[],[]]
 * 解释:
 * 输入输出各部分都应该是链表，而不是数组。
 * 例如, 输入的结点 root 的 val= 1, root.next.val = 2, \root.next.next.val = 3, 且
 * root.next.next.next = null。
 * 第一个输出 output[0] 是 output[0].val = 1, output[0].next = null。
 * 最后一个元素 output[4] 为 null, 它代表了最后一个部分为空链表。
 *
 *
 * 示例 2：
 *
 *
 * 输入:
 * root = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], k = 3
 * 输出: [[1, 2, 3, 4], [5, 6, 7], [8, 9, 10]]
 * 解释:
 * 输入被分成了几个连续的部分，并且每部分的长度相差不超过1.前面部分的长度大于等于后面部分的长度。
 *
 *
 *
 *
 * 提示:
 *
 *
 * root 的长度范围： [0, 1000].
 * 输入的每个节点的大小范围：[0, 999].
 * k 的取值范围： [1, 50].
 *
 *
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        // 首先获取链表的长度
        let len = Self::get_len(root.as_ref());
        // 计算每小块的长度
        let split_piece = len / k;
        let mut remain = len % k;
        let mut chunk_len = vec![];
        for _ in 0..k {
            let piece = if remain > 0 {
                remain -= 1;
                1
            } else {
                0
            } + split_piece;
            chunk_len.push(piece);
        }
        // 剩下的就是根据chunk_len来分隔链表, 怎么分啊。。
        // 只能通过创建链表来分了= =
        let mut head = root;
        let mut cur = head.as_mut();
        //let mut dummy_head = None;
        let mut res = vec![];
        for len in chunk_len {
            // 创建len个结点的链表
            let mut dummy = Some(Box::new(ListNode::new(0)));
            let mut dummy_ref = dummy.as_mut();
            let mut tmp_len = 0;
            while let Some(node) = cur {
                dummy_ref.as_mut().unwrap().next = Some(Box::new(ListNode::new(node.val)));
                dummy_ref = dummy_ref.unwrap().next.as_mut();
                cur = node.next.as_mut();
                tmp_len += 1;
                if tmp_len == len {
                    break;
                }
            }
            res.push(dummy.unwrap().next.take());
        }
        res
    }

    pub fn get_len(head: Option<&Box<ListNode>>) -> i32 {
        let mut head = head;
        let mut len = 0;
        while let Some(node) = head {
            head = node.next.as_ref();
            len += 1;
        }
        len
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l725() {
        assert_eq!(
            vec![
                ListNode::to_list(vec![1]),
                ListNode::to_list(vec![2]),
                ListNode::to_list(vec![3]),
                None,
                None
            ],
            Solution::split_list_to_parts(ListNode::to_list(vec![1, 2, 3]), 5)
        );
        assert_eq!(
            vec![
                ListNode::to_list(vec![1, 2, 3, 4]),
                ListNode::to_list(vec![5, 6, 7]),
                ListNode::to_list(vec![8, 9, 10]),
            ],
            Solution::split_list_to_parts(
                ListNode::to_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
                3
            )
        );

        assert_eq!(
            vec![None, None, None],
            Solution::split_list_to_parts(None, 3)
        );
    }
}
