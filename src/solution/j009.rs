use super::Solution;

// 用两个栈实现队列, 插入时，将数据放在full里
// 弹出时，将full中的数据倒入empty,并将empty中的第一个数据弹出，再将empty的数据弹入full中
// 但这个方法并不优秀，比较优秀的方法是
// 当第二个栈空的时候，再将第一个栈中的元素弹到第二个栈中，这样就不用两个栈中的元素来回倒腾
// 而不空的时候，直接弹出即可
struct CQueue {
    full: Vec<i32>,
    empty: Vec<i32>,
}

impl CQueue {
    fn new() -> Self {
        CQueue {
            full: Vec::new(),
            empty: Vec::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.full.push(value)
    }

    fn delete_head(&mut self) -> i32 {
        if self.empty.is_empty() {
            while let Some(v) = self.full.pop() {
                self.empty.push(v);
            }
        }
        self.empty.pop().unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_000() {
        let mut q = CQueue::new();
        q.append_tail(1);
        q.append_tail(2);
        q.append_tail(3);
        assert_eq!(q.delete_head(), 1);
        assert_eq!(q.delete_head(), 2);
        assert_eq!(q.delete_head(), 3);
        assert_eq!(q.delete_head(), -1);
    }
}
