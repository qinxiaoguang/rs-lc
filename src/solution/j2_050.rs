use super::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    // 计算任意路径的和等于target_sum的个数
    // 使用前缀和来解，在dfs遍历的时候，同时记录根节点到当前节点的所有路径前缀和对应的个数
    // 之后再在前缀和中寻找target_sum - now_val的值即可(这样可以计算出以当前节点为终点的所有路径和为target_sum的个数)
    pub fn path_sum_v2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut m = HashMap::new();
        m.insert(0, 1);
        let mut cnt = 0;
        Self::_path_sum(root, target_sum, 0, &mut m, &mut cnt);
        return cnt;
    }

    fn _path_sum(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        sum: i64,
        map: &mut HashMap<i64, i32>,
        cnt: &mut i32,
    ) {
        if root.is_none() {
            return;
        }
        let r = root.unwrap();
        let val = r.borrow().val;
        let now_sum = sum + val as i64;
        if let Some(&tmpcnt) = map.get(&(now_sum - target_sum as i64)) {
            *cnt += tmpcnt;
        }

        // 计算前缀和
        *map.entry(now_sum).or_insert(0) += 1;
        Self::_path_sum(r.borrow().left.clone(), target_sum, now_sum, map, cnt);
        Self::_path_sum(r.borrow().right.clone(), target_sum, now_sum, map, cnt);
        // 减去前缀和
        *map.entry((now_sum) as i64).or_insert(0) -= 1;

        return;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_050() {}
}
