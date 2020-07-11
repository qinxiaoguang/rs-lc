use super::{ListNode, TreeNode};
pub struct Solution {}
/*
 * @lc app=leetcode.cn id=105 lang=rust
 *
 * [105] 从前序与中序遍历序列构造二叉树
 *
 * https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/
 *
 * algorithms
 * Medium (64.43%)
 * Likes:    371
 * Dislikes: 0
 * Total Accepted:    50.2K
 * Total Submissions: 77.8K
 * Testcase Example:  '[3,9,20,15,7]\n[9,3,15,20,7]'
 *
 * 根据一棵树的前序遍历与中序遍历构造二叉树。
 *
 * 注意:
 * 你可以假设树中没有重复的元素。
 *
 * 例如，给出
 *
 * 前序遍历 preorder = [3,9,20,15,7]
 * 中序遍历 inorder = [9,3,15,20,7]
 *
 * 返回如下的二叉树：
 *
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 前序遍历的特点是，第一个结点必是根结点，而左结点所有值一定在右结点所有值的左边
        // 中序结点的特点是，根结点的左边就是左结点的所有值，同样右边就是右结点的所有值
        // 利用这两个特点，每次找到根结点，组合成树就ok
        Self::build_tree_v2(&preorder, &inorder)
    }

    pub fn build_tree_v2(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let (mid_pos, _) = inorder
            .iter()
            .enumerate()
            .find(|(_, x)| x == &&preorder[0])
            .unwrap();

        Some(Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: Self::build_tree_v2(&preorder[1..(mid_pos + 1)], &inorder[0..mid_pos]),
            right: Self::build_tree_v2(&preorder[mid_pos + 1..], &inorder[mid_pos + 1..]),
        })))
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l105() {
        println!(
            "{:?}",
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
        );

        println!("{:?}", Solution::build_tree(vec![], vec![]));
    }
}
