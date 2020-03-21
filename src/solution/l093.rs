pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 从字符串中找出合法的ip地址
    // 如25525511135 = > ["255.255.11.135", "255.255.111.35"]
    // 使用dfs即可，最大深度也就4
    // 要注意的点是 当选到的字符是 "00"时，要退出
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        // 即插入.的位置
        let mut res = vec![];
        Self::dfs(&s, &mut vec![], &mut res);
        res
    }

    fn dfs(s: &str, tmp: &mut Vec<String>, res: &mut Vec<String>) {
        if tmp.len() == 4 && s.len() == 0 {
            res.push(tmp.clone().join("."));
            return;
        }
        if tmp.len() > 4 {
            return;
        }
        let mut ip;
        for i in 0..s.len() {
            let ip_str = s[0..=i].to_string();
            ip = ip_str.parse::<i32>().unwrap_or(-1);
            if ip > 255 || ip < 0 || (ip_str.len() >= 2 && ip_str.starts_with("0")) {
                break;
            }
            tmp.push(ip_str);
            Self::dfs(&s[i + 1..], tmp, res);
            tmp.pop();
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l093() {
        println!(
            "{:?}",
            Solution::restore_ip_addresses("25525511135".to_string())
        );

        println!("{:?}", Solution::restore_ip_addresses("010010".to_string()));
    }
}
