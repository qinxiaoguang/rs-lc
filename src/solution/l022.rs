pub struct Solution {}
impl Solution {
    // 给出n代表n对括号，输出这n对括号组成的所有可能的有效括号对数.其实就是dfs，并且保证前边的左括号的个数一定是大于等于右括号的个数
    #![allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        Self::generate("".to_string(), 0, 0, 2 * (n as i64), &mut res);
        res
    }

    fn generate(now_string: String, left: i64, right: i64, sum: i64, res: &mut Vec<String>) {
        if left < right || left + right > sum {
            return;
        } else if left == right && left + right == sum {
            res.push(now_string);
        } else {
            let left_string = format!("{}(", now_string);
            let right_string = format!("{})", now_string);
            Self::generate(left_string, left + 1, right, sum, res);
            Self::generate(right_string, left, right + 1, sum, res);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l022() {
        println!("{:?}", Solution::generate_parenthesis(0));
        println!("{:?}", Solution::generate_parenthesis(1));
        println!("{:?}", Solution::generate_parenthesis(2));
        println!("{:?}", Solution::generate_parenthesis(3));
    }
}
