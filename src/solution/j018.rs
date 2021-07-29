use super::Solution;
use super::*;

impl Solution {
    // rust里面链表题挺烦，此题最好使用递归来实现
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if let Some(mut v) = head {
            if v.val == val {
                return v.next.take();
            } else {
                v.next = Self::delete_node(v.next.take(), val);
                return Some(v);
            }
        }

        return None;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j018() {
        println!(
            "Solution::delete_node(ListNode::to_list(vec![3,1,2]), 1) = {:?}",
            Solution::delete_node(ListNode::to_list(vec![3, 1, 2]), 1)
        );
    }
}
