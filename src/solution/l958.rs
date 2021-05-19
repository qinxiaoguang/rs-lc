use super::TreeNode;
pub struct Solution {}
/*
 * @lc app=leetcode.cn id=958 lang=rust
 *
 * [958] 二叉树的完全性检验
 *
 * https://leetcode-cn.com/problems/check-completeness-of-a-binary-tree/description/
 *
 * algorithms
 * Medium (50.31%)
 * Likes:    70
 * Dislikes: 0
 * Total Accepted:    8.8K
 * Total Submissions: 17.4K
 * Testcase Example:  '[1,2,3,4,5,6]'
 *
 * 给定一个二叉树，确定它是否是一个完全二叉树。
 *
 * 百度百科中对完全二叉树的定义如下：
 *
 * 若设二叉树的深度为 h，除第 h 层外，其它各层 (1～h-1) 的结点数都达到最大个数，第 h
 * 层所有的结点都连续集中在最左边，这就是完全二叉树。（注：第 h 层可能包含 1~ 2^h 个节点。）
 *
 *
 *
 * 示例 1：
 *
 *
 *
 * 输入：[1,2,3,4,5,6]
 * 输出：true
 * 解释：最后一层前的每一层都是满的（即，结点值为 {1} 和 {2,3} 的两层），且最后一层中的所有结点（{4,5,6}）都尽可能地向左。
 *
 *
 * 示例 2：
 *
 *
 *
 * 输入：[1,2,3,4,5,null,7]
 * 输出：false
 * 解释：值为 7 的结点没有尽可能靠向左侧。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 树中将会有 1 到 100 个结点。
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
    // 为每个节点附上编号，根结点编号为1
    // 每个节点的左节点编号为2*i，右结点编号为2*i + 1
    // 进行宽搜时，编号是一个无间隔的升序，则其是一个完全二叉树
    // 或者最后一个节点(第n个节点)是编号n，则其就是一个完全二叉树
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = vec![(root, 1)];
        let mut seq = vec![];
        let mut size = 0;
        while !queue.is_empty() {
            let (node, n) = queue.remove(0);
            if node.is_none() {
                continue;
            }
            seq.push(n);
            size += 1;
            let true_node = node.unwrap();
            let left_node = true_node.borrow_mut().left.take();
            let right_node = true_node.borrow_mut().right.take();
            queue.push((left_node, n * 2));
            queue.push((right_node, n * 2 + 1));
        }
        seq.pop().unwrap() == size
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l958() {}
}
