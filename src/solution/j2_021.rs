use super::Solution;
use super::*;

impl Solution {
    // 删除链表倒数第n个节点
    // 正常思路是使用快慢指针来解
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len: i32 = 0;
        {
            let mut p = dummy_head.as_ref();
            while p.unwrap().next.is_some() {
                len += 1;
                p = p.unwrap().next.as_ref();
            }
        }
        let idx = len - n;
        {
            let mut p = dummy_head.as_mut();
            for _ in 0..(idx) {
                p = p.unwrap().next.as_mut();
            }
            let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = next;
        }
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_021() {}
}
