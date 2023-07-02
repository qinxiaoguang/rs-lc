use super::*;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // 获取所有路径，并计算和
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Self::_sum_numbers(root, 0, &mut res);
        res
    }

    // 传入当前路径，使用dfs
    pub fn _sum_numbers(root: Option<Rc<RefCell<TreeNode>>>, mut sum: i32, res: &mut i32) {
        if let Some(r) = root {
            let v = r.borrow().val;
            let tmpsum = sum * 10 + v;
            if r.borrow().left.is_none() && r.borrow().right.is_none() {
                *res += tmpsum;
                return;
            }
            Self::_sum_numbers(r.borrow().left.clone(), tmpsum, res);
            Self::_sum_numbers(r.borrow().right.clone(), tmpsum, res);
        }
        return;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_049() {}
}
