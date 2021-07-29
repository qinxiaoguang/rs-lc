use super::Solution;
use super::*;
use std::borrow::Borrow;

impl Solution {
    // 广搜
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut q = std::collections::VecDeque::new();
        let mut res = vec![];
        q.push_back(root);
        while let Some(node) = q.pop_front() {
            if let Some(n) = node {
                res.push(n.as_ref().borrow().val);
                q.push_back(n.as_ref().borrow_mut().left.take());
                q.push_back(n.as_ref().borrow_mut().right.take());
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j032() {
        let t = TreeNode::to_tree(&[1, 2, 3, 4, 5]);
        println!("Solution::level_order(t) = {:?}", Solution::level_order(t));
    }
}
