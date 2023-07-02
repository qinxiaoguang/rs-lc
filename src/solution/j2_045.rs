use super::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    // 找出二叉树最底层最左边的值
    // bfs即解
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q: VecDeque<(i32, Rc<RefCell<TreeNode>>)> = VecDeque::new();
        let mut cur_deep = 0;
        let mut cur_res = root.clone().unwrap().borrow().val;
        q.push_back((0, root.clone().unwrap()));

        while let Some(pop) = q.pop_front() {
            if cur_deep != pop.0 {
                cur_deep = pop.0;
                cur_res = pop.1.borrow().val;
            }

            if let Some(left) = pop.1.borrow_mut().left.as_ref() {
                q.push_back((pop.0 + 1, left.clone()));
            }
            if let Some(right) = pop.1.borrow_mut().right.as_ref() {
                q.push_back((pop.0 + 1, right.clone()));
            }
        }

        return cur_res;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_045() {}
}
