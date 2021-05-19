use super::Solution;
use super::TreeNode;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 只考虑根结点 + 则根结点有两种状态，要么被盗，要么不盗
        // 那么根结点+子结点的状态有: 根盗 + 子不盗， 根不盗 则分三种状态: 子左盗，子右盗，两子都盗
        // 利用一个数组来保存计算好的值，防止下次遍历时重复计算
        // 但是写完代码后，答案不对，网上看别人写的代码只考虑了两种情况
        // 根盗+ 四个孙子盗 / 根不盗 + 两个儿子盗
        // 这种写法能得出最终解，但是有点不明白，为什么只有这两种情况呢，四个孙子为什么不能只有两个孙子盗，两个不盗呢？
        // oh shit
        /*let mut steal_root = HashMap::new();
        let mut no_steal_root = HashMap::new();

        let res = std::cmp::max(
            Self::get_best(root.as_ref(), false, 1, &mut steal_root, &mut no_steal_root),
            Self::get_best(root.as_ref(), true, 1, &mut steal_root, &mut no_steal_root),
        );*/
        let res = Self::get_best_2(root.as_ref(), 1, &mut HashMap::new());
        res
    }

    fn get_best_2(
        root: Option<&Rc<RefCell<TreeNode>>>,
        node_num: i32,
        mem: &mut HashMap<i32, i32>,
    ) -> i32 {
        if root.is_none() {
            return 0;
        }
        if let Some(n) = mem.get(&node_num) {
            return *n;
        }
        let r = root.unwrap().as_ref().borrow();
        let do_it = r.val
            + if r.left.is_none() {
                0
            } else {
                Self::get_best_2(
                    r.left.as_ref().unwrap().as_ref().borrow().left.as_ref(),
                    node_num * 2 * 2,
                    mem,
                ) + Self::get_best_2(
                    r.left.as_ref().unwrap().as_ref().borrow().right.as_ref(),
                    node_num * 2 * 2 + 1,
                    mem,
                )
            }
            + if r.right.is_none() {
                0
            } else {
                Self::get_best_2(
                    r.right.as_ref().unwrap().as_ref().borrow().left.as_ref(),
                    (node_num * 2 + 1) * 2,
                    mem,
                ) + Self::get_best_2(
                    r.right.as_ref().unwrap().as_ref().borrow().right.as_ref(),
                    (node_num * 2 + 1) * 2 + 1,
                    mem,
                )
            };
        let not_do = Self::get_best_2(r.left.as_ref(), node_num * 2, mem)
            + Self::get_best_2(r.right.as_ref(), node_num * 2 + 1, mem);

        let res = std::cmp::max(do_it, not_do);
        mem.insert(node_num, res);
        return res;
    }

    fn get_best(
        root: Option<&Rc<RefCell<TreeNode>>>,
        steal: bool,
        node_num: i32,
        steal_root: &mut HashMap<i32, i32>,
        no_steal_root: &mut HashMap<i32, i32>,
    ) -> i32 {
        let (left_num, right_num) = (node_num * 2, node_num * 2 + 1);
        if root.is_none() {
            return 0;
        }
        let mut cache = if steal {
            steal_root.get(&node_num)
        } else {
            no_steal_root.get(&node_num)
        };

        if cache.is_some() {
            return *cache.unwrap();
        }

        match root {
            Some(r) => {
                if steal {
                    // root 盗，子不盗
                    let left_val = Self::get_best(
                        r.as_ref().borrow().left.as_ref(),
                        false,
                        left_num,
                        steal_root,
                        no_steal_root,
                    );
                    let right_val = Self::get_best(
                        r.as_ref().borrow().right.as_ref(),
                        false,
                        right_num,
                        steal_root,
                        no_steal_root,
                    );
                    let res = r.as_ref().borrow().val + left_val + right_val;
                    steal_root.insert(node_num, res);
                    res
                } else {
                    // root不盗, 子左盗
                    let mut tmp_min = std::cmp::max(
                        Self::get_best(
                            r.as_ref().borrow().left.as_ref(),
                            true,
                            left_num,
                            steal_root,
                            no_steal_root,
                        ) + Self::get_best(
                            r.as_ref().borrow().right.as_ref(),
                            false,
                            right_num,
                            steal_root,
                            no_steal_root,
                        ),
                        Self::get_best(
                            r.as_ref().borrow().left.as_ref(),
                            false,
                            left_num,
                            steal_root,
                            no_steal_root,
                        ) + Self::get_best(
                            r.as_ref().borrow().right.as_ref(),
                            true,
                            right_num,
                            steal_root,
                            no_steal_root,
                        ),
                    );

                    // 两子盗
                    tmp_min = std::cmp::max(
                        Self::get_best(
                            r.as_ref().borrow().left.as_ref(),
                            true,
                            left_num,
                            steal_root,
                            no_steal_root,
                        ) + Self::get_best(
                            r.as_ref().borrow().right.as_ref(),
                            true,
                            right_num,
                            steal_root,
                            no_steal_root,
                        ),
                        tmp_min,
                    );
                    no_steal_root.insert(node_num, tmp_min);
                    tmp_min
                }
            }
            None => 0,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_337() {
        let node = TreeNode::to_tree(&vec![1, 2, 3]);
        assert_eq!(Solution::rob(node), 5);

        let node = TreeNode::to_tree(&vec![3, 4, 5, 1, 3, -1, 1]);
        assert_eq!(Solution::rob(node), 9);
        assert_eq!(
            Solution::rob(TreeNode::to_tree(&vec![3, 2, 3, -1, 3, -1, 1])),
            7
        );

        assert_eq!(Solution::rob(TreeNode::to_tree(&vec![1, 2, -1])), 2);
        assert_eq!(Solution::rob(TreeNode::to_tree(&vec![1, -1, 2])), 2);
        assert_eq!(Solution::rob(TreeNode::to_tree(&vec![1, -1, -1])), 1);
        assert_eq!(
            Solution::rob(TreeNode::to_tree(&vec![
                41, 37, 44, 24, 39, 42, 48, 1, 35, 38, 40, -1, 43, 46, 49, 0, 2, 30, 36, -1, -1,
                -1, -1, -1, -1, 45, 47, -1, -1, -1, -1, -1, 4, 29, 32, -1, -1, -1, -1, -1, -1, 3,
                9, 26, -1, 31, 34, -1, -1, 7, 11, 25, 27, -1, -1, 33, -1, 6, 8, 10, 16, -1, -1, -1,
                28, -1, -1, 5, -1, -1, -1, -1, -1, 15, 19, -1, -1, -1, -1, 12, -1, 18, 20, -1, 13,
                17, -1, -1, 22, -1, 14, -1, -1, 21, 23
            ])),
            690
        );
    }
}
