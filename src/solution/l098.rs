pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #![allow(dead_code)]
    #[inline]
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
    // 判断二叉树是否是二叉搜索树
    // 注意 节点的右子树的所有值 都要小于当前节点
    // 最简单的解法是:中序遍历为升序。。。
    // 竟然还傻不垃圾的回溯了。。。
    #![allow(dead_code)]
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid(root, (std::i64::MIN, std::i64::MAX))
    }

    pub fn is_valid(root: Option<Rc<RefCell<TreeNode>>>, (min, max): (i64, i64)) -> bool {
        if let Some(node) = root {
            let left_rc = Rc::clone(&node);
            let right_rc = Rc::clone(&node);
            let left = &left_rc.borrow().left;
            let right = &right_rc.borrow().right;
            let val = node.borrow().val;
            println!("{}", val);
            let (mut left_res, mut right_res) = (true, true);
            if let Some(left_node) = left {
                let left_val = left_node.borrow().val;
                if left_val >= val || left_val as i64 >= max || left_val as i64 <= min {
                    return false;
                }
                left_res = Self::is_valid(
                    Some(Rc::clone(left_node)),
                    (min, std::cmp::min(max, val as i64)),
                );
            }
            if let Some(right_node) = right {
                let right_val = right_node.borrow().val;
                if right_val <= val || right_val as i64 <= min || right_val as i64 >= max {
                    return false;
                }
                right_res = Self::is_valid(
                    Some(Rc::clone(right_node)),
                    (std::cmp::max(min, val as i64), max),
                );
            }
            return left_res && right_res;
        }
        return true;
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn test_l098() {
        println!("");
        println!("");
    }
}
