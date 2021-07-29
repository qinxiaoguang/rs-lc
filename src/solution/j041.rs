use super::Solution;

// 利用两个堆来存储数据
// 其中一个小顶堆，一个大顶堆
// 其中小顶堆保存大数，大顶堆保存小数
// 插入完成后，对两个堆数据进行平衡
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    small: BinaryHeap<Reverse<i32>>, // 小顶堆, 需要借助Reverse
    big: BinaryHeap<i32>,            // 大顶堆
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            big: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        loop {
            if let Some(v) = self.small.peek() {
                if num > (*v).0 {
                    // 放到大顶堆
                    self.small.push(Reverse(num));
                    break;
                }
            }
            if let Some(v) = self.big.peek() {
                if num < *v {
                    self.big.push(num);
                    break;
                }
            }
            // 两个都没找到, 则加到数少的那里面
            if self.small.len() <= self.big.len() {
                self.small.push(Reverse(num));
            } else {
                self.big.push(num);
            }
            break;
        }
        // small的堆顶值要比big大
        while let (Some(s), Some(b)) = (self.small.peek(), self.big.peek()) {
            if (*s).0 < *b {
                self.small.push(Reverse(self.big.pop().unwrap()));
            } else {
                break;
            }
        }
        // 调整， 将堆长度大的放在堆长度小的里面
        while (self.small.len() as i32 - self.big.len() as i32) >= 2 {
            self.big.push(self.small.pop().unwrap().0);
        }

        while (self.big.len() as i32 - self.small.len() as i32) >= 2 {
            self.small.push(Reverse(self.big.pop().unwrap()));
        }
    }

    fn find_median(&mut self) -> f64 {
        if self.big.len() == self.small.len() {
            (*self.big.peek().unwrap_or(&0) + (*self.small.peek().unwrap_or(&Reverse(0))).0) as f64
                / 2.0
        } else if self.big.len() > self.small.len() {
            *self.big.peek().unwrap_or(&0) as f64
        } else {
            (*self.small.peek().unwrap_or(&Reverse(0))).0 as f64
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j041() {
        let mut m = MedianFinder::new();
        m.add_num(1);
        m.add_num(2);
        assert_eq!(m.find_median(), 1.5);
        m.add_num(3);
        assert_eq!(m.find_median(), 2.0);
    }
}
