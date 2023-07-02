use super::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    // 找出二叉树每一层的最大值
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut q: VecDeque<(i32, Rc<RefCell<TreeNode>>)> = VecDeque::new();
        let mut res = vec![];
        q.push_back((0, root.clone().unwrap()));

        while let Some(pop) = q.pop_front() {
            if res.len() == pop.0 as usize {
                res.push(pop.1.borrow().val);
            } else {
                res[pop.0 as usize] = res[pop.0 as usize].max(pop.1.borrow().val);
            }
            if let Some(left) = pop.1.borrow().left.as_ref() {
                q.push_back((pop.0 + 1, left.clone()));
            }

            if let Some(right) = pop.1.borrow().right.as_ref() {
                q.push_back((pop.0 + 1, right.clone()));
            }
        }

        return res;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_044() {}
}
