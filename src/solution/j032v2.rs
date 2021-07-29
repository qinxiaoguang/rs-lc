use super::Solution;
use super::*;

impl Solution {
    // 只是在遍历的时候 加上层数就行了
    pub fn level_order_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q = std::collections::VecDeque::new();
        let mut res = vec![];
        q.push_back((1, root));
        while let Some((deep, node)) = q.pop_front() {
            if let Some(n) = node {
                while res.len() < deep {
                    res.push(vec![]);
                }
                res[deep - 1].push(n.as_ref().borrow().val);
                q.push_back((deep + 1, n.as_ref().borrow_mut().left.take()));
                q.push_back((deep + 1, n.as_ref().borrow_mut().right.take()));
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j032v2() {
        let t = TreeNode::to_tree(&[1, 2, 3, 4, -1, -1, 5, 6, 7]);

        println!(
            "Solution::level_order(t) = {:?}",
            Solution::level_order_v2(t)
        );
    }
}
