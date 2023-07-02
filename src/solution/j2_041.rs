use std::collections::VecDeque;
struct MovingAverage {
    size: usize,
    sum: i32,
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        Self {
            size: size as usize,
            sum: 0,
            queue: VecDeque::new(),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.queue.len() == self.size {
            self.sum -= self.queue.pop_front().unwrap();
        }

        self.queue.push_back(val);
        self.sum += val;
        self.sum as f64 / self.queue.len() as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_041() {}
}
