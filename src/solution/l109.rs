use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=109 lang=rust
 *
 * [109] 有序链表转换二叉搜索树
 *
 * https://leetcode-cn.com/problems/convert-sorted-list-to-binary-search-tree/description/
 *
 * algorithms
 * Medium (70.69%)
 * Likes:    161
 * Dislikes: 0
 * Total Accepted:    21.3K
 * Total Submissions: 30.1K
 * Testcase Example:  '[-10,-3,0,5,9]'
 *
 * 给定一个单链表，其中的元素按升序排序，将其转换为高度平衡的二叉搜索树。
 *
 * 本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。
 *
 * 示例:
 *
 * 给定的有序链表： [-10, -3, 0, 5, 9],
 *
 * 一个可能的答案是：[0, -3, 9, -10, null, 5], 它可以表示下面这个高度平衡二叉搜索树：
 *
 * ⁠     0
 * ⁠    / \
 * ⁠  -3   9
 * ⁠  /   /
 * ⁠-10  5
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
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 通过快慢指针找到中间结点，该结点作为当前父结点
        // 并将链表断为左右两个链表，递归调用此方法
        // 因链表不好操作，应将链表转换成vec后再进行操作
        let mut vec = vec![];
        let mut head = head;
        while let Some(mut node) = head {
            vec.push(node.val);
            head = node.next.take()
        }
        Self::vec_to_bst(&vec)
    }

    pub fn vec_to_bst(vec: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() {
            return None;
        }
        // 数组转换为二叉搜索树的方式是先查找到中间结点作为根结点，并将分隔后的左右两个数组进行分治调用
        let mid = vec.len() / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: vec[vec.len() / 2],
            left: Self::vec_to_bst(&vec[..mid]),
            right: Self::vec_to_bst(&vec[mid + 1..]),
        })))
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l109() {
        println!(
            "{:?}",
            Solution::sorted_list_to_bst(ListNode::to_list(vec![1, 2, 3]))
        )
    }
}
