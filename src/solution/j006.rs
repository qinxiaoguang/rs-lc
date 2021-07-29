use super::Solution;
use super::*;

impl Solution {
    // 倒着返回所有值
    // 就是用栈
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut head = head;
        while let Some(n) = head {
            stack.push(n.val);
            head = n.next;
        }
        stack.reverse();
        stack
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j006() {
        let mut first = ListNode::new(1);
        let mut sec = Some(Box::new(ListNode::new(2)));
        first.next = sec;
        let mut head = Some(Box::new(first));
        println!("res:{:?}", Solution::reverse_print(head));
    }
}
