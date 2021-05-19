use super::Solution;
use super::TreeNode;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone, Copy)]
pub enum Order {
    PreOrder,
    InOrder,
    PostOrder,
}

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if p.is_none() || q.is_none() {
            return None;
        }
        let p = p.unwrap().borrow_mut().val;
        let q = q.unwrap().borrow_mut().val;
        // 需要前序和中序来获取结果
        // 既然是二叉树，必然需要考虑到三个序的特性
        // 使用前序加中序
        let mut pre_order = Vec::new();
        Self::pre_order(root.clone(), &mut pre_order);
        let mut inner_order = Vec::new();
        Self::inner_order(root.clone(), &mut inner_order);

        println!("root = {:?}", root);
        println!(
            "pre_order = {:?}, inner order = {:?}",
            pre_order, inner_order
        );
        Some(Rc::new(RefCell::new(TreeNode::new(Self::get_res(
            &pre_order,
            &inner_order,
            p,
            q,
        )))))
    }

    // 获取最小公共祖先
    fn get_res(pre_order: &[i32], in_order: &[i32], p: i32, q: i32) -> i32 {
        let mut pre_v = vec![];
        let mut in_v = vec![];
        for i in 0..pre_order.len() {
            pre_v.push(pre_order[i]);
            if pre_order[i] == p || pre_order[i] == q {
                break;
            }
        }
        let mut find = 0;
        for i in 0..in_order.len() {
            in_v.push(in_order[i]);
            if in_order[i] == p || in_order[i] == q {
                match find {
                    0 => find += 1,
                    1 => break,
                    _ => {}
                };
            }
        }
        // 找pre_v和in_v中最先出现的数字
        for v in pre_v {
            if in_v.contains(&v) {
                return v;
            }
        }
        0
    }

    fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        Self::order(root, order, Order::PreOrder)
    }

    fn inner_order(root: Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        Self::order(root, order, Order::InOrder)
    }

    fn order(root: Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>, ord: Order) {
        match root.as_ref() {
            Some(r) => {
                let left = r.borrow_mut().left.take();
                let right = r.borrow_mut().right.take();
                if ord == Order::PreOrder {
                    order.push(r.borrow_mut().val);
                }
                Self::order(left.clone(), order, ord);
                if ord == Order::InOrder {
                    order.push(r.borrow_mut().val);
                }
                Self::order(right.clone(), order, ord);
                if ord == Order::PostOrder {
                    order.push(r.borrow_mut().val);
                }
                r.borrow_mut().left = left;
                r.borrow_mut().right = right;
            }
            None => {}
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn test_l236() {
        let r = TreeNode::to_tree(&vec![3, 5, 1, 6, 2, 0, 8, -1, -1, 7, 4]);
        let left = TreeNode::to_tree(&vec![5]);
        let right = TreeNode::to_tree(&vec![4]);
        let res = left.clone();
        assert_eq!(Solution::lowest_common_ancestor(r, left, right), res);

        let r = TreeNode::to_tree(&vec![3, 5, 1, 6, 2, 0, 8, -1, -1, 7, 4]);
        let left = TreeNode::to_tree(&vec![0]);
        let right = TreeNode::to_tree(&vec![8]);
        let res = TreeNode::to_tree(&vec![1]);
        assert_eq!(Solution::lowest_common_ancestor(r, left, right), res); // my res:3, right res 1

        let r = TreeNode::to_tree(&vec![1, 2, 3, 4, 5, 6, 7]);
        let left = TreeNode::to_tree(&vec![1]);
        let right = TreeNode::to_tree(&vec![2]);

        println!(
            "res is = {:?}",
            Solution::lowest_common_ancestor(r, left, right)
        );
    }
}
