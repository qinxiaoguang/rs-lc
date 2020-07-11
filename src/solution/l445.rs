use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=445 lang=rust
 *
 * [445] 两数相加 II
 *
 * https://leetcode-cn.com/problems/add-two-numbers-ii/description/
 *
 * algorithms
 * Medium (54.49%)
 * Likes:    230
 * Dislikes: 0
 * Total Accepted:    43.4K
 * Total Submissions: 75.4K
 * Testcase Example:  '[7,2,4,3]\n[5,6,4]'
 *
 * 给你两个 非空 链表来代表两个非负整数。数字最高位位于链表开始位置。它们的每个节点只存储一位数字。将这两数相加会返回一个新的链表。
 *
 * 你可以假设除了数字 0 之外，这两个数字都不会以零开头。
 *
 *
 *
 * 进阶：
 *
 * 如果输入链表不能修改该如何处理？换句话说，你不能对列表中的节点进行翻转。
 *
 *
 *
 * 示例：
 *
 * 输入：(7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4)
 * 输出：7 -> 8 -> 0 -> 7
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }
        // 不让反转链表来计算两个链表相加的结果
        // 用链表太难写了，索性转成vec来实现
        let (mut l1, mut l2) = (l1, l2);
        let (mut len1, mut len2) = (Self::get_len(l1.as_ref()), Self::get_len(l2.as_ref()));
        let mut zero_head = Some(Box::new(ListNode::new(0)));
        let mut zero_head_ref = zero_head.as_mut();
        for _ in 0..(len1 - len2).abs() {
            // 补足前边的0
            zero_head_ref.as_mut()?.next = Some(Box::new(ListNode::new(0)));
            zero_head_ref = zero_head_ref?.next.as_mut();
        }
        let mut list2 = None;
        zero_head_ref?.next = if len1 < len2 {
            list2 = l2;
            l1
        } else {
            list2 = l1;
            l2
        };
        let mut list1 = zero_head?.next.take();

        // 将list1,list2两个链表的值进行相加
        let mut add = vec![];
        let mut sum = vec![0];
        while let (Some(mut node1), Some(mut node2)) = (list1, list2) {
            let mut val = node1.val + node2.val;
            if val >= 10 {
                add.push(val / 10);
                val = val % 10;
            } else {
                add.push(0);
            }
            sum.push(val);
            list1 = node1.next.take();
            list2 = node2.next.take();
        }
        add.push(0);

        // 不能用数字加和的方式 ，因为会溢出
        let len = sum.len();
        let mut sum: Vec<i32> = sum.into_iter().rev().collect();
        let mut add: Vec<i32> = add.into_iter().rev().collect();
        let mut sum2 = vec![];
        let mut add2 = vec![0];
        for (i, v) in sum.iter().enumerate() {
            let tmp = v + add[i];
            if tmp >= 10 {
                add2.push(1);
                sum2.push(tmp - 10);
            } else {
                add2.push(0);
                sum2.push(tmp);
            }
        }
        // 对sum2 及 add2进行加和操作
        let mut res_sum = vec![];
        let mut add_one = false;
        sum2.into_iter()
            .zip(add2.into_iter())
            .into_iter()
            .for_each(|(sum, add)| {
                let mut tmp = sum + add + if add_one { 1 } else { 0 };
                if tmp >= 10 {
                    add_one = true;
                    tmp -= 10;
                } else {
                    add_one = false;
                }
                res_sum.push(tmp);
            });
        if add_one {
            res_sum.push(1);
        }
        //println!("res_sum is:{:?}", res_sum);

        let res_sum_rev: Vec<i32> = res_sum.into_iter().rev().collect();
        // 找到第一个不是0的位置，如果是None,则有可能全不为0，也有可能全为0
        let pos = if res_sum_rev.iter().all(|&x| x == 0) {
            // 全0，则pos为最后一位
            res_sum_rev.len() - 1
        } else {
            res_sum_rev.iter().position(|&x| x != 0).unwrap_or(0)
        };

        //println!("pos is:{:?}", pos);
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut res_ref = res.as_mut();
        for v in res_sum_rev.into_iter().skip(pos) {
            res_ref.as_mut()?.next = Some(Box::new(ListNode::new(v)));
            res_ref = res_ref?.next.as_mut();
        }

        res?.next.take()
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
    fn test_l445() {
        assert_eq!(
            ListNode::to_list(vec![1, 3, 6, 8]),
            Solution::add_two_numbers(
                ListNode::to_list(vec![1, 2, 3]),
                ListNode::to_list(vec![1, 2, 4, 5])
            )
        );

        assert_eq!(
            ListNode::to_list(vec![1, 3, 6, 8]),
            Solution::add_two_numbers(None, ListNode::to_list(vec![1, 3, 6, 8]))
        );

        assert_eq!(None, Solution::add_two_numbers(None, None));

        assert_eq!(
            ListNode::to_list(vec![7, 8, 0, 7]),
            Solution::add_two_numbers(
                ListNode::to_list(vec![7, 2, 4, 3]),
                ListNode::to_list(vec![5, 6, 4])
            )
        );

        assert_eq!(
            ListNode::to_list(vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 6]),
            Solution::add_two_numbers(
                ListNode::to_list(vec![3, 9, 9, 9, 9, 9, 9, 9, 9, 9]),
                ListNode::to_list(vec![7])
            )
        );
        assert_eq!(
            ListNode::to_list(vec![0]),
            Solution::add_two_numbers(ListNode::to_list(vec![0]), ListNode::to_list(vec![0]))
        )
    }
}
