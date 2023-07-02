use std::collections::VecDeque;

struct RecentCounter {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        while !self.queue.is_empty() && self.queue[0] < t - 3000 {
            self.queue.pop_front();
        }

        return self.queue.len() as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_041() {}
}
