use super::Solution;

use std::{collections::HashMap, mem};

struct Node {
    next: Option<Box<Node>>,
    pre: Option<Box<Node>>,
    val: i32,
}
struct LRUCache {
    m: HashMap<i32, Node>,
    head: Node,
    tail: Node,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 *
 *
 * 使用hashMap + 双向链表实现
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            m: HashMap::with_capacity(capacity as usize),
            head: Node {
                pre: None,
                next: None,
                val: 0,
            },
            tail: Node {
                pre: None,
                next: None,
                val: 0,
            },
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.m.contains_key(&key) {
            return -1;
        }

        // 如果获取的值刚好是头节点，什么操作都不需要
        // 交换当前节点的前后指针
        let mut node = self.m.get_mut(&key);
        unimplemented!();
        // 交换头节点
        mem::swap(&mut node.as_mut().unwrap().next, &mut self.head.next);

        self.head.next.as_ref().unwrap().val
    }

    //
    fn put(&mut self, key: i32, value: i32) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_030() {}
}
