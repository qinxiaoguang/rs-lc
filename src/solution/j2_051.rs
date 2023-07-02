use super::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    // 最大路径和，其实就是求左节点根的最大路径和 + 右节点的根的最大路径和
    // 需要注意路径为负的情况，直接舍弃就好
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = i32::MIN;
        Self::_max_path_sum(root, &mut ans);
        ans
    }

    fn _max_path_sum(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let r = root.unwrap();
        let val = r.borrow().val;
        let lmax = Self::_max_path_sum(r.borrow().left.clone(), ans);
        let rmax = Self::_max_path_sum(r.borrow().right.clone(), ans);

        *ans = (*ans).max(if lmax < 0 { 0 } else { lmax } + if rmax < 0 { 0 } else { rmax } + val);
        let max = lmax.max(rmax);
        if max < 0 {
            return val;
        }
        return lmax.max(rmax) + val;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_051() {}
}
