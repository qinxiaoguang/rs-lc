use super::TreeNode;
pub struct Solution {}
/*
 * @lc app=leetcode.cn id=951 lang=rust
 *
 * [951] 翻转等价二叉树
 *
 * https://leetcode-cn.com/problems/flip-equivalent-binary-trees/description/
 *
 * algorithms
 * Medium (64.96%)
 * Likes:    54
 * Dislikes: 0
 * Total Accepted:    6K
 * Total Submissions: 9.2K
 * Testcase Example:  '[1,2,3,4,5,6,null,null,null,7,8]\n[1,3,2,null,6,4,5,null,null,null,null,8,7]'
 *
 * 我们可以为二叉树 T 定义一个翻转操作，如下所示：选择任意节点，然后交换它的左子树和右子树。
 *
 * 只要经过一定次数的翻转操作后，能使 X 等于 Y，我们就称二叉树 X 翻转等价于二叉树 Y。
 *
 * 编写一个判断两个二叉树是否是翻转等价的函数。这些树由根节点 root1 和 root2 给出。
 *
 *
 *
 * 示例：
 *
 * 输入：root1 = [1,2,3,4,5,6,null,null,null,7,8], root2 =
 * [1,3,2,null,6,4,5,null,null,null,null,8,7]
 * 输出：true
 * 解释：我们翻转值为 1，3 以及 5 的三个节点。
 *
 *
 *
 *
 *
 * 提示：
 *
 *
 * 每棵树最多有 100 个节点。
 * 每棵树中的每个值都是唯一的、在 [0, 99] 范围内的整数。
 *
 *
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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root1.is_none() && root2.is_none() {
            return true;
        }
        // 第一种方法是，两个树同时按照左节点小于右节点来反转，最后相等则为true,不等则为false
        // 第二种方法是递归，若为true,则第一个树的小节点一定与第二棵树的小节点互为反转树
        // 讲真，rust写树的题也太别扭了
        if let (Some(l), Some(r)) = (root1, root2) {
            if l.borrow().val != r.borrow().val {
                return false;
            }
            // l的小节点
            let l1 = l.borrow_mut().left.clone(); // 只能用clone,用take()的话将改变树的结构，回溯的时候就可能错误
            let l2 = l.borrow_mut().right.clone();
            let r1 = r.borrow_mut().left.clone();
            let r2 = r.borrow_mut().right.clone();
            let mut res1 = vec![];
            let mut res2 = vec![];
            res1.push(Self::flip_equiv(l1.clone(), r1.clone()));
            res1.push(Self::flip_equiv(l1, r2.clone()));
            res2.push(Self::flip_equiv(l2.clone(), r1));
            res2.push(Self::flip_equiv(l2, r2));
            return (res1[0] || res1[1]) && (res2[0] || res2[1]);
        }
        return false;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l951() {
        let eight = TreeNode::new(8);
        let seven = TreeNode::new(7);
        let four = TreeNode::new(4);
        let mut two = TreeNode::new(2);
        let mut one = TreeNode::new(1);
        let mut six = TreeNode::new(6);
        let mut three = TreeNode::new(3);
        three.add_left(six);
        let mut five = TreeNode::new(5);
        five.add_left(seven).add_right(eight);
        two.add_left(four).add_right(five);
        one.add_left(two).add_right(three);
        let left = Some(Rc::new(RefCell::new(one)));

        let eight2 = TreeNode::new(8);
        let seven2 = TreeNode::new(7);
        let four2 = TreeNode::new(4);
        let mut two2 = TreeNode::new(2);
        let mut one2 = TreeNode::new(1);
        let mut six2 = TreeNode::new(6);
        let mut three2 = TreeNode::new(3);
        three2.add_right(six2);
        let mut five2 = TreeNode::new(5);
        five2.add_right(seven2).add_left(eight2);
        two2.add_left(four2).add_right(five2);
        one2.add_right(two2).add_left(three2);
        let right = Some(Rc::new(RefCell::new(one2)));

        //println!("left:{:?}\nright:{:?}", left, right);

        Solution::flip_equiv(left, right);
    }
}
