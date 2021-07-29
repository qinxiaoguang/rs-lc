use super::Solution;
use crate::solution::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    // 求某树是不是平衡二叉树
    // 平衡二叉树的要点是
    // 左右节点的深度差只差1
    // 注意与平衡搜索二叉树做区分
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::j055v2_dfs(root.as_ref()).1
    }

    // 返回深度，传入最小值和最大值
    fn j055v2_dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        if let Some(node) = root {
            let v = node.borrow().val;
            let (ldeep, lbalance) = Self::j055v2_dfs(node.borrow().left.as_ref());
            if !lbalance {
                return (0, false);
            }
            let (rdeep, rbalance) = Self::j055v2_dfs(node.borrow().right.as_ref());
            if !rbalance {
                return (0, false);
            }
            if (rdeep - ldeep).abs() > 1 {
                return (0, false);
            }
            return (if rdeep > ldeep { rdeep } else { ldeep } + 1, true);
        }

        return (0, true);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j055v2() {
        assert_eq!(
            Solution::is_balanced(TreeNode::to_tree(&[3, 9, 20, -1, -1, 15, 7])),
            true
        );
        assert_eq!(
            Solution::is_balanced(TreeNode::to_tree(&[1, 2, 2, 3, 3, -1, -1, 4, 4])),
            false
        );
    }
}
