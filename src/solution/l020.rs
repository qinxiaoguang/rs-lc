pub struct Solution {}
impl Solution {
    // 判断输入的括号是否合法
    // 采用栈
    pub fn is_valid(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut stack = vec![];
        use std::collections::HashMap;
        let mut table = HashMap::new();
        table.insert(')', '(');
        table.insert(']', '[');
        table.insert('}', '{');
        for c in chars.iter() {
            if table.contains_key(c) {
                if let Some(pop_c) = stack.pop() {
                    if pop_c != table.get(c).unwrap() {
                        return false;
                    }
                    continue;
                }
                return false;
            } else {
                stack.push(c);
            }
        }
        stack.len() == 0
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l020() {
        assert_eq!(true, Solution::is_valid("()".to_string()));
        assert_eq!(true, Solution::is_valid("".to_string()));
        assert_eq!(true, Solution::is_valid("()[]{}".to_string()));
        assert_eq!(true, Solution::is_valid("{[]}".to_string()));
        assert_eq!(false, Solution::is_valid("(".to_string()));
        assert_eq!(false, Solution::is_valid("([)]".to_string()));
    }
}
