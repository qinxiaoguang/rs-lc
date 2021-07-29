use super::Solution;
use super::*;
use std::cell::Ref;

impl Solution {
    // 获取所有的路径和等于target的路径, 深搜即可
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::j034_dfs(root.as_ref(), target, &mut vec![], 0, &mut res);
        res
    }

    fn j034_dfs(
        root: Option<&Rc<RefCell<TreeNode>>>,
        target: i32,
        now_path: &mut Vec<i32>,
        sum: i32,
        res: &mut Vec<Vec<i32>>,
    ) {
        if let Some(v) = root {
            now_path.push(v.borrow().val);
            let sum = sum + v.borrow().val;
            if target == sum && v.borrow().left.is_none() && v.borrow().right.is_none() {
                res.push(now_path.clone());
                now_path.pop();
                return;
            }
            Self::j034_dfs(v.borrow().left.as_ref(), target, now_path, sum, res);
            Self::j034_dfs(v.borrow().right.as_ref(), target, now_path, sum, res);
            now_path.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j034() {
        let res = Solution::path_sum(
            TreeNode::to_tree(&[5, 4, 8, 11, -1, 13, 4, 7, 2, -1, -1, -1, -1, 5, 1]),
            22,
        );
        println!("res = {:?}", res);

        let res = Solution::path_sum(TreeNode::to_tree(&[-2, -1, -3]), -5);
        println!("res = {:?}", res);
    }
}
