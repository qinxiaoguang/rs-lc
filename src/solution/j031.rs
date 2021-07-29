use super::Solution;

impl Solution {
    // 经典题，判断另一个序列是不是第一个序列的压栈弹栈的一个结果
    // 因为栈中所有元素均不相等，所以模拟即可
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut i = 0;
        for v in pushed {
            stack.push(v);
            while stack.len() != 0 && stack[stack.len() - 1] == popped[i] {
                i += 1;
                stack.pop();
            }
        }
        stack.len() == 0 && i == popped.len()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j031() {
        assert!(Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 5, 3, 2, 1]
        ));
        assert!(Solution::validate_stack_sequences(vec![1], vec![1]))
    }
}
