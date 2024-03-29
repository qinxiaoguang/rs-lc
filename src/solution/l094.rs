pub struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    // 二叉树的中序遍历
    // 题目想让你通过迭代的方式去实现
    // 迭代其实就是通过栈, 有点麻烦，有点难啊= =
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Solution::inorder_traverse(root.as_ref(), &mut (|v| res.push(v)));
        res
    }

    fn inorder_traverse<F: FnMut(i32)>(root: Option<&Rc<RefCell<TreeNode>>>, consumer: &mut F) {
        if let Some(node) = root {
            Solution::inorder_traverse(node.borrow().left.as_ref(), consumer);
            consumer(root.as_ref().unwrap().borrow().val);
            Solution::inorder_traverse(node.borrow().right.as_ref(), consumer);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l094() {
        let node = TreeNode::new(3);
        let mut two = TreeNode::new(2);
        two.left = Some(Rc::new(RefCell::new(node)));
        let mut one = TreeNode::new(1);
        one.right = Some(Rc::new(RefCell::new(two)));
        println!(
            "{:?}",
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(one))))
        );
    }
}
