use super::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // 把所有子节点都是0的树剪掉。dfs即可
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 左右子节点都是0，则都剪掉
        if root.is_none() {
            return root;
        }

        let mut l_res = None;
        let mut r_res = None;

        if let Some(l) = root.as_ref().unwrap().borrow().left.as_ref() {
            l_res = Self::prune_tree(Some(l.clone()));
        }

        if let Some(r) = root.as_ref().unwrap().borrow().right.as_ref() {
            r_res = Self::prune_tree(Some(r.clone()));
        }

        if root.as_ref().unwrap().borrow().val == 0 && l_res == None && r_res == None {
            return None;
        }

        root.as_ref().unwrap().borrow_mut().left = l_res;
        root.as_ref().unwrap().borrow_mut().right = r_res;
        return root;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_047() {}
}
