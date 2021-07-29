use super::Solution;
use super::*;

impl Solution {
    // 输入前序和中序构建二叉树
    // 只要给中序，剩下的不管前还是后，都能构造出二叉树
    // 前序第一个节点必定是根结点，而在中序中找到该结点，则左侧的在根结点左侧，右侧在根结点右侧
    // 这样就构造成子问题，构建左右两个节点树
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_ref(&preorder, &inorder)
    }

    fn build_tree_ref(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let mut root = TreeNode {
            val: preorder[0],
            left: None,
            right: None,
        };
        let mut idx = 0usize;
        while inorder[idx] != preorder[0] {
            idx += 1;
        }
        if idx != 0 {
            root.left = Self::build_tree_ref(&preorder[1..idx + 1], &inorder[0..idx]);
        }
        if idx != preorder.len() - 1 {
            root.right = Self::build_tree_ref(&preorder[idx + 1..], &inorder[idx + 1..]);
        }
        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j007() {
        let t = Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        println!("t = {:?}", t);
    }
}
