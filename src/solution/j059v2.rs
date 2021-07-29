use super::Solution;

use std::collections::VecDeque;
struct MaxQueue {
    down: VecDeque<i32>,
    queue: VecDeque<i32>,
}

// 要求均摊复杂度是O(1)
// push_back和pop_front好说
// 问题是max_value
// 与上一题有异曲同工之妙，只需要一个辅助单调递减队列即可解决
impl MaxQueue {
    fn new() -> Self {
        MaxQueue {
            down: VecDeque::new(),
            queue: VecDeque::new(),
        }
    }

    fn max_value(&mut self) -> i32 {
        if let Some(v) = self.down.front() {
            return *v;
        }
        return -1;
    }

    fn push_back(&mut self, value: i32) {
        self.queue.push_back(value);
        while let Some(v) = self.down.back() {
            if *v < value {
                self.down.pop_back();
            } else {
                self.down.push_back(value);
                break;
            }
        }
        if self.down.len() == 0 {
            self.down.push_back(value);
        }
    }

    fn pop_front(&mut self) -> i32 {
        if let Some(v) = self.queue.pop_front() {
            if let Some(front) = self.down.front() {
                if *front == v {
                    self.down.pop_front();
                }
            }
            return v;
        }
        return -1;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j059v2() {
        let mut m = MaxQueue::new();
        m.push_back(1);
        m.push_back(3);
        m.push_back(2);
        assert_eq!(m.max_value(), 3);
        m.pop_front();
        assert_eq!(m.max_value(), 3);
        m.pop_front();
        assert_eq!(m.max_value(), 2);
        m.pop_front();
        assert_eq!(m.max_value(), -1);
        m.push_back(3);
        m.push_back(3);
        m.push_back(1);
        assert_eq!(m.max_value(), 3);
        m.pop_front();
        assert_eq!(m.max_value(), 3);
        m.pop_front();
        assert_eq!(m.max_value(), 1);
    }
}
