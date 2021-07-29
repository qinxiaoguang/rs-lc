use super::Solution;

// push, pop ,min的时间复杂度都是O(1)
// 正常的栈的Push和pop都是O(1)
// 所以重点是min如何变为O(1)
// 一般找最小值比较好的思路，就是利用树，但是其插入却需要logn的时间复杂度
// 所以不行
// 这个题，可以借用辅助栈，辅助栈中，只往里添加比栈顶小的元素，使得改栈内的元素始终是降序排列的
// 而取出的时候，只需要判断，取出的数据是否和辅助栈的栈顶数据相等 ，若相等，辅助栈也取出即可
// 那么min只需要取辅助栈的栈顶元素即可
struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.min.len() == 0 || x <= self.min[self.min.len() - 1] {
            self.min.push(x);
        }
    }

    fn pop(&mut self) {
        if let Some(v) = self.stack.pop() {
            if v == self.min[self.min.len() - 1] {
                self.min.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn min(&self) -> i32 {
        self.min[self.min.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j030() {
        let mut ms = MinStack::new();
        ms.push(0);
        ms.push(1);
        ms.push(0);
        assert_eq!(ms.min(), 0);
        ms.pop();
        assert_eq!(ms.min(), 0);
    }
}
