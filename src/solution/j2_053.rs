use super::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // 查找p的中序后继节点
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || p.is_none() {
            return None;
        }

        return Self::_inorder_successor(root, p.unwrap().borrow().val, &mut false);
    }

    fn _inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        pval: i32,
        occur: &mut bool,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let r = root.unwrap();
        let lres = Self::_inorder_successor(r.borrow().left.clone(), pval, occur);
        if lres.is_some() {
            return lres;
        }
        if *occur {
            return Some(r);
        }
        // 中序遍历
        if r.borrow().val == pval {
            *occur = true;
        }
        println!("{},{},{}", r.borrow().val, pval, occur);
        let rres = Self::_inorder_successor(r.borrow().right.clone(), pval, occur);
        if rres.is_some() {
            return rres;
        }
        return None;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_053() {}
}
