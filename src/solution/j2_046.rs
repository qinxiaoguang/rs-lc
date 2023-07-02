use super::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut q: VecDeque<(i32, Rc<RefCell<TreeNode>>)> = VecDeque::new();
        let mut cur_deep = 0;
        let mut res = vec![];
        res.push(root.clone().unwrap().borrow().val);
        q.push_back((0, root.clone().unwrap()));

        while let Some(pop) = q.pop_front() {
            if cur_deep != pop.0 {
                cur_deep = pop.0;
                res.push(pop.1.borrow().val);
            }
            if let Some(right) = pop.1.borrow_mut().right.as_ref() {
                q.push_back((pop.0 + 1, right.clone()));
            }
            if let Some(left) = pop.1.borrow_mut().left.as_ref() {
                q.push_back((pop.0 + 1, left.clone()));
            }
        }

        return res;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_046() {}
}
