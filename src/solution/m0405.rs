use super::Solution;
use super::TreeNode;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    // 注意点，需要判断左侧最大值，小于父节点，右侧最小值，大于父节点，其他没啥
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_ref(root.as_ref()).0
    }

    // 返回该节点中的最小值以及最大值
    pub fn is_valid_bst_ref(root: Option<&Rc<RefCell<TreeNode>>>) -> (bool, i32, i32) {
        // 如果是的话，左右节点也是
        if root.is_none() {
            return (true, -1, -1);
        }

        let root = root.as_ref().unwrap();
        let v = root.as_ref().borrow().val;
        if let Some(l) = root.as_ref().borrow().left.as_ref() {
            if l.as_ref().borrow().val >= v {
                return (false, -1, -1);
            }
        }
        if let Some(r) = root.as_ref().borrow().right.as_ref() {
            if r.as_ref().borrow().val <= v {
                return (false, -1, -1);
            }
        }

        let (left_valid, left_min_v, left_max_v) =
            Self::is_valid_bst_ref(root.as_ref().borrow().left.as_ref());
        let (right_valie, right_min_v, right_max_v) =
            Self::is_valid_bst_ref(root.as_ref().borrow().right.as_ref());

        let mut max_v = v;
        let mut min_v = v;
        max_v = max_v.max(left_max_v);
        max_v = max_v.max(right_max_v);
        min_v = if left_min_v != -1 {
            min_v.min(left_min_v)
        } else {
            min_v
        };
        min_v = if right_min_v != -1 {
            min_v.min(right_min_v)
        } else {
            min_v
        };
        (
            left_valid
                && right_valie
                && if left_max_v != -1 {
                    left_max_v < v
                } else {
                    true
                }
                && if right_min_v != -1 {
                    right_min_v > v
                } else {
                    true
                },
            min_v,
            max_v,
        )
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_m0405() {
        let t = TreeNode::to_tree(&vec![2, 1, 3]);
        println!("{:?}", t);
        assert_eq!(Solution::is_valid_bst(t), true);
    }
}
