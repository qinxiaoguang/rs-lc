use super::Solution;

impl Solution {
    // 二叉搜索树的定义是左边比根节点小，右边比根节点大
    // 而后序遍历的一个特征是，根节点在最后一个值出现。
    // 所以找到比根结点大的数，并判断其是否符合条件
    // 剩下的就是比根结点小的数，同样判断其是否符合条件即可
    pub fn verify_postorder(postorder: Vec<i32>) -> bool {
        Self::verify_postorder_ref(&postorder, i32::MIN, i32::MAX)
    }
    // min和max是为了限制该值
    pub fn verify_postorder_ref(postorder: &[i32], min: i32, max: i32) -> bool {
        if postorder.len() == 0 {
            return true;
        }

        let root_val = *postorder.last().unwrap();

        if root_val <= min || root_val >= max {
            return false;
        }
        for i in (0..postorder.len() - 1).rev() {
            if postorder[i] <= min || postorder[i] >= max {
                return false;
            }
            if postorder[i] < root_val {
                // 找到临界点
                return Self::verify_postorder_ref(&postorder[0..=i], min, root_val)
                    && Self::verify_postorder_ref(
                        &postorder[i + 1..postorder.len() - 1],
                        root_val,
                        max,
                    );
            }
        }
        // 没找到，前边的值都比root_val大
        Self::verify_postorder_ref(&postorder[0..postorder.len() - 1], root_val, max)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j033() {
        assert_eq!(Solution::verify_postorder(vec![1, 3, 2, 6, 5]), true);
        assert_eq!(Solution::verify_postorder(vec![1, 6, 3, 2, 5]), false);
    }
}
