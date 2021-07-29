use super::Solution;
use super::*;

impl Solution {
    // 求二叉树的深度，可以深搜，也可以宽搜
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Self::j055v1_dfs(root.as_ref(), 1, &mut res);
        res
    }

    fn j055v1_dfs(root: Option<&Rc<RefCell<TreeNode>>>, now_deep: i32, max_deep: &mut i32) {
        if let Some(node) = root {
            *max_deep = (*max_deep).max(now_deep);
            Self::j055v1_dfs(node.borrow().left.as_ref(), now_deep + 1, max_deep);
            Self::j055v1_dfs(node.borrow().right.as_ref(), now_deep + 1, max_deep);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j055v1() {
        assert_eq!(
            Solution::max_depth(TreeNode::to_tree(&[3, 9, 20, -1, -1, 15, 7])),
            3
        );
    }
}
