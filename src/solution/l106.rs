use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=106 lang=rust
 *
 * [106] 从中序与后序遍历序列构造二叉树
 *
 * https://leetcode-cn.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/
 *
 * algorithms
 * Medium (67.01%)
 * Likes:    169
 * Dislikes: 0
 * Total Accepted:    27.4K
 * Total Submissions: 40.7K
 * Testcase Example:  '[9,3,15,20,7]\n[9,15,7,20,3]'
 *
 * 根据一棵树的中序遍历与后序遍历构造二叉树。
 *
 * 注意:
 * 你可以假设树中没有重复的元素。
 *
 * 例如，给出
 *
 * 中序遍历 inorder = [9,3,15,20,7]
 * 后序遍历 postorder = [9,15,7,20,3]
 *
 * 返回如下的二叉树：
 *
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 *
 *
 */

// @lc code=start
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 后序遍历的特点是最后一个点必然是根结点
        // 与前+中 推树是一样的思路。
        // 只有中配合前或后任意一个才能推出二叉树，而前+后则推不出。
        // 原因就是前就是后的一个变种
        Self::build_tree_v2(&inorder, &postorder)
    }

    pub fn build_tree_v2(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.len() == 0 {
            return None;
        }
        let (mid_pos, _) = inorder
            .iter()
            .enumerate()
            .find(|(_, x)| x == &&postorder[postorder.len() - 1])
            .unwrap();

        Some(Rc::new(RefCell::new(TreeNode {
            val: postorder[postorder.len() - 1],
            left: Self::build_tree_v2(&inorder[0..mid_pos], &postorder[0..mid_pos]),
            right: Self::build_tree_v2(
                &inorder[mid_pos + 1..],
                &postorder[mid_pos..postorder.len() - 1],
            ),
        })))
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l106() {
        println!(
            "{:?}",
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
        );

        println!("{:?}", Solution::build_tree(vec![], vec![]));
    }
}
