use super::Solution;
use super::*;
use std::borrow::{Borrow, BorrowMut};

impl Solution {
    pub fn mirror_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 每个节点进行旋转即可
        if let Some(mut r) = root {
            let left = r.as_ref().borrow_mut().left.take();
            let right = r.as_ref().borrow_mut().right.take();
            r.as_ref().borrow_mut().right = Self::mirror_tree(left);
            r.as_ref().borrow_mut().left = Self::mirror_tree(right);
            return Some(r);
        }
        None
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j027() {
        println!(
            " = {:?}",
            Solution::mirror_tree(TreeNode::to_tree(&[4, 2, 7, 1, 3, 6, 9]))
        );
    }
}
