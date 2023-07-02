use super::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // 倒序的中序遍历
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_convert_bst(root.clone(), &mut 0);
        return root;
    }

    fn _convert_bst(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(r) = root {
            Self::_convert_bst(r.borrow().right.clone(), sum);
            *sum += r.borrow().val;
            // 将该节点值替换为sum
            r.borrow_mut().val = *sum;
            Self::_convert_bst(r.borrow().left.clone(), sum);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_054() {}
}
