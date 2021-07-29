use super::Solution;
use super::*;

impl Solution {
    // 只需要dfs一次
    pub fn is_sub_structure(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // 遍历a, 找到a的root和b相等的点
        Self::is_sub_structure_ref(a.as_ref(), b.as_ref())
    }

    pub fn is_sub_structure_ref(
        a: Option<&Rc<RefCell<TreeNode>>>,
        b: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // 遍历a, 找到a的root和b相等的点
        if let (Some(a), Some(b)) = (a, b) {
            if a.borrow().val == b.borrow().val && Self::tree_equal(Some(a), Some(b)) {
                return true;
            } else {
                // a != b
                if Self::is_sub_structure_ref(a.borrow().left.as_ref(), Some(b))
                    || Self::is_sub_structure_ref(a.borrow().right.as_ref(), Some(b))
                {
                    return true;
                }
            }
        }
        a.is_none() && b.is_none()
    }

    fn tree_equal(a: Option<&Rc<RefCell<TreeNode>>>, b: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if let (Some(a), Some(b)) = (a, b) {
            if a.borrow().val == b.borrow().val {
                // 检查左节点和右节点
                return Self::tree_equal(a.borrow().left.as_ref(), b.borrow().left.as_ref())
                    && Self::tree_equal(a.borrow().right.as_ref(), b.borrow().right.as_ref());
            }
            return false;
        }
        b.is_none()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_000() {
        let a = TreeNode::to_tree(&vec![1, 2, 3]);
        let b = TreeNode::to_tree(&vec![3, 1]);
        println!("a = {:?}", a);
        println!("b = {:?}", b);
        println!(
            "Solution::is_sub_structure(a,b) = {:?}",
            Solution::is_sub_structure(a, b)
        );
    }
}
