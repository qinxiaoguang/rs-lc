pub struct Solution {}
impl Solution {
    // 输入 : LEETCODEISHIRING, 3，表示三行z字型打印
    // 用三个数组，数组中存储字符串，模拟即可。
    #![allow(dead_code)]
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut res: Vec<String> = vec!["".to_string(); num_rows as usize];
        let (mut now_row, mut flag) = (0, -1);
        for c in chars {
            if now_row == 0 || now_row == num_rows - 1 {
                flag = -flag;
            }
            res[now_row as usize].push(c);
            now_row += flag;
        }
        let mut res_string = String::with_capacity(s.len());
        res.iter().for_each(|s| res_string.push_str(&s));
        res_string
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l006() {
        assert_eq!(
            "LCIRETOESIIGEDHN",
            Solution::convert("LEETCODEISHIRING".to_string(), 3)
        );

        assert_eq!(
            "LDREOEIIECIHNTSG",
            Solution::convert("LEETCODEISHIRING".to_string(), 4)
        );
    }
}
