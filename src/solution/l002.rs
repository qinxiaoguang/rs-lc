#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    #![allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4) 输出：7 -> 0 -> 8
        let (mut l1, mut l2) = (l1, l2);
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut now = &mut res;
        let (mut l1_end, mut l2_end) = (false, false);
        let mut over_flow = false;
        loop {
            let left_num = match l1 {
                Some(v) => {
                    l1 = v.next;
                    v.val
                }
                None => {
                    l1_end = true;
                    0
                }
            };
            let right_num = match l2 {
                Some(v) => {
                    l2 = v.next;
                    v.val
                }
                None => {
                    l2_end = true;
                    0
                }
            };
            if l1_end && l2_end && !over_flow {
                break res.unwrap().next;
            }
            let sum = left_num + right_num + if over_flow { 1 } else { 0 };
            let sum = if sum >= 10 {
                over_flow = true;
                sum - 10
            } else {
                over_flow = false;
                sum
            };
            // 因为执行完now.unwrap()后，now中的option<>的值就会被消耗，这个unwrap()方法是unwrap(self)
            // 所以需要使用as_mut将option<T>修改为option<&mut T>
            now.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            now = &mut now.as_mut().unwrap().next;
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_l002() {
        use super::ListNode;
        let mut l1 = Some(Box::new(ListNode::new(2)));
        let mut tail = &mut l1;
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        tail = &mut tail.as_mut().unwrap().next;
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        let mut l2 = Some(Box::new(ListNode::new(5)));
        let mut tail = &mut l2;
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
        tail = &mut tail.as_mut().unwrap().next;
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

        let res = super::Solution::add_two_numbers(l1, l2);
        println!("{:?}", res);
    }
}
