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
    // 卡特兰数
    // 其应用一般有: 1. 入栈出栈的方式的个数 2.合法左右括号组成的括号种类数 3. 此题，生成二插搜索树
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        println!("{}", n);
        vec![]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l095() {
        Solution::generate_trees(1);
    }
}
