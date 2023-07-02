use super::Solution;
use super::*;
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    // 波兰表达式
    // 使用栈进行运算
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        tokens.iter().for_each(|x| match x.as_str() {
            "+" => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                stack.push(num1 + num2);
            }
            "-" => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                stack.push(num2 - num1);
            }
            "*" => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                stack.push(num2 * num1);
            }
            "/" => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                stack.push(num2 / num1);
            }
            _ => {
                let num = x.parse::<i32>().unwrap();
                stack.push(num);
            }
        });
        return stack.pop().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_036() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }
}
