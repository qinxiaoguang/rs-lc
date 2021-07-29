use super::Solution;
use super::*;

impl Solution {
    // 使用双指针，先找第k个，然后两个双一起往前走
    // 但是rust双指针太难了, 只能用栈
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let (res, deep) = Self::get_kth_from_end_v2(head, k);
        res
    }

    // 返回节点和深度,深度用来判断是不是倒数第k个
    pub fn get_kth_from_end_v2(
        head: Option<Box<ListNode>>,
        k: i32,
    ) -> (Option<Box<ListNode>>, i32) {
        let mut head = head;
        if let Some(mut v) = head {
            let (last, deep) = Self::get_kth_from_end_v2(v.next.take(), k);
            return if deep >= k {
                (last, deep + 1)
            } else {
                v.next = last;
                (Some(v), deep + 1)
            };
        }

        (head, 0)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j022() {
        println!(
            "Solution::get_kth_from_end(ListNode::to_list(vec![1,2,3,4]), 1) = {:?}",
            Solution::get_kth_from_end(ListNode::to_list(vec![1, 2, 3, 4]), 0)
        );
    }
}
