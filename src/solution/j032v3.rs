use super::Solution;
use super::*;

impl Solution {
    // 之字形打印,方法很多
    // 1. 在v2的版本上，对第偶数行做下reverse即可
    // 2. 使用双端队列存储数据，若奇数层，则将数据放到前边，若偶数层，将数据放到后边
    // 3. 每次处理2层，第一层按这个顺序，第二层按另一个顺序
    pub fn level_order_v3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
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
        for i in 0..res.len() {
            if i & 1 == 1 {
                res[i].reverse();
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j032v3() {
        let t = TreeNode::to_tree(&[1, 2, 3, 4, 5]);
        println!(
            "Solution::level_order(t) = {:?}",
            Solution::level_order_v3(t)
        );
    }
}
