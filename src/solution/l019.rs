// Definition for singly-linked list.
use super::ListNode;
pub struct Solution {}
impl Solution {
    // 删除链表中倒数第n个节点
    // 一次遍历 即采用快慢指针，但是此题的快慢指针并不是快慢，
    // 而是先让第一指针移动n+1步，后再让慢指针开始移动，保持两个指针的距离恒定为n即可.
    // 快慢指针写不出来，借鉴leetcode的已通过的代码，用的递归的形式实现。
    // 实现的思路就是递归每次深次递归的时候 都会保存其深度，在退出递归的时候，会判断当前的深度是否符合条件
    // 符合条件的则，则将对应的节点变换为下下个节点。因为不存在两个指针，所以可以采用一个mut来搞。
    // 而快慢指针需要一个mut一个ref,或者两个mut，不符合rust情况。
    // 有点扯淡
    #![allow(dead_code)]
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Solution::remove(head, n).0
    }

    fn remove(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
        if let Some(node) = head {
            let (removed, cnt) = Solution::remove(node.next, n);
            if cnt + 1 == n {
                (removed, cnt + 1)
            } else {
                let mut new_node = Box::new(ListNode::new(node.val));
                new_node.next = removed;
                (Some(new_node), cnt + 1)
            }
        } else {
            (None, 0)
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l019() {
        use super::ListNode;
        let mut l1 = Some(Box::new(ListNode::new(1)));
        let mut tail = &mut l1;
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        tail = &mut tail.as_mut().unwrap().next;
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        tail = &mut tail.as_mut().unwrap().next;
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        tail = &mut tail.as_mut().unwrap().next;
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));
        println!("{:?}", Solution::remove_nth_from_end(l1, 2));
    }
}
