use super::Solution;
use super::TreeNode;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

impl Solution {
    // 二叉树和链表的题，用rust做就是受折磨
    // 没通，不做了！题目描述不清！反正是dfs的题！
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let mut res = Some(Rc::new(RefCell::new(TreeNode {
            left: root,
            right: None,
            val: 0,
        })));
        Self::l1080_dfs(res.as_mut(), limit, 0);
        res.unwrap().as_ref().borrow_mut().left.take()
    }

    // 返回该结点根叶结点的最大值,以及该结点是不是无用结点
    // sum是root前的点的和
    fn l1080_dfs(root: Option<&mut Rc<RefCell<TreeNode>>>, limit: i32, sum: i32) -> (i32, bool) {
        if root.is_none() {
            return (0, true);
        }
        let root = root.unwrap();
        let root_val = root.as_ref().borrow_mut().val;
        let mut left = root.as_ref().borrow_mut().left.take();
        let mut right = root.as_ref().borrow_mut().right.take();
        let (left_val, left_is_useless) = Self::l1080_dfs(left.as_mut(), limit, sum + root_val);
        let (right_val, right_is_useless) = Self::l1080_dfs(right.as_mut(), limit, sum + root_val);
        if !left_is_useless {
            root.as_ref().borrow_mut().left = left;
        }
        if !right_is_useless {
            root.as_ref().borrow_mut().right = right;
        }
        let max = root_val
            + if left_val > right_val {
                left_val
            } else {
                right_val
            };
        let max_val = sum + max;
        (max, if max_val < limit { true } else { false })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1080() {}
}
