use super::Solution;
use super::*;

impl Solution {
    // 二叉搜索树，找第k大的节点
    // 二叉搜索树的中序遍历，就是排序的
    // 利用该规则去找即可
    // 但注意问题是找第k大的节点
    pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut res = Vec::new();
        let mut k = k;
        Self::j054_inner_order(root.as_ref(), &mut res);
        res[res.len() - k as usize]
    }

    fn j054_inner_order(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::j054_inner_order(node.borrow().left.as_ref(), res);
            res.push(node.borrow().val);
            Self::j054_inner_order(node.borrow().right.as_ref(), res);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j054() {
        assert_eq!(
            Solution::kth_largest(TreeNode::to_tree(&[3, 1, 4, -1, 2]), 1),
            4
        );
    }
}
