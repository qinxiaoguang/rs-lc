use super::Solution;
use super::*;
use std::borrow::{Borrow, BorrowMut};

impl Solution {
    // 注意 二叉树对称 不表示其子树也是对称的
    // 只需要进行递归的比较 两个节点是否是对称的
    // 两个结点是对称的条件是a.val == b.val且 递归的判断 symmetric(a.left, b.right) && symmetric(a.right, b.left)
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root.as_ref() {
            return Self::symmetric(
                root.as_ref().borrow().left.as_ref(),
                root.as_ref().borrow().right.as_ref(),
            );
        }
        true
    }

    fn symmetric(
        left: Option<&Rc<RefCell<TreeNode>>>,
        right: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let (Some(l), Some(r)) = (left, right) {
            return l.as_ref().borrow().val == r.as_ref().borrow().val
                && Self::symmetric(
                    l.as_ref().borrow().left.as_ref(),
                    r.as_ref().borrow().right.as_ref(),
                )
                && Self::symmetric(
                    l.as_ref().borrow().right.as_ref(),
                    r.as_ref().borrow().left.as_ref(),
                );
        }
        left.is_none() && right.is_none()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j028() {
        let a = TreeNode::to_tree(&[1, 2, 2, 3, 4, 4, 3]);
        println!(
            "Solution::is_symmetric(a) = {:?}",
            Solution::is_symmetric(a)
        );
    }
}
