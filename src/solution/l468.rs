use super::Solution;
use std::num::ParseIntError;

impl Solution {
    // 判断ip是否合法，是的话返回IPv4/6, 否则返回Neither
    pub fn valid_ip_address(ip: String) -> String {
        // 可以使用状态机，但是状态机写起来太长，所以只使用暴力法来解
        let mut blocks: Vec<&str> = ip.split(".").collect();
        if blocks.len() == 1 {
            blocks = ip.split(":").collect();
        }
        match blocks.len() {
            4 => {
                for block in blocks.iter() {
                    match block.parse::<i32>().map(|v| v >= 0 && v <= 255) {
                        Ok(true) => {
                            // 需要判断block是否合法
                            if block.is_empty() || block.len() >= 2 && block.starts_with("0") {
                                return String::from("Neither");
                            }
                        }
                        _ => return String::from("Neither"),
                    }
                }
                String::from("IPv4")
            }
            8 => {
                for block in blocks.iter() {
                    //println!("block = {:?}", block);
                    if block.len() > 4 || block.len() == 0 {
                        return String::from("Neither");
                    }
                    let lowercase = block.to_lowercase();
                    for c in lowercase.chars() {
                        match c {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a'
                            | 'b' | 'c' | 'd' | 'e' | 'f' => continue,
                            _ => return String::from("Neither"),
                        }
                    }
                }
                String::from("IPv6")
            }
            _ => String::from("Neither"),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid_ip_address() {
        assert_eq!(
            Solution::valid_ip_address(String::from("172.16.254.255")),
            String::from("IPv4")
        );
        assert_eq!(
            Solution::valid_ip_address(String::from("172.16.254.1")),
            String::from("IPv4")
        );
        assert_eq!(
            Solution::valid_ip_address(String::from("2001:0db8:85a3:0:0:8A2E:0370:7334")),
            String::from("IPv6")
        );
        assert_eq!(
            Solution::valid_ip_address(String::from("256.256.256.256")),
            String::from("Neither")
        );
        assert_eq!(
            Solution::valid_ip_address(String::from("1e1.4.5.6")),
            String::from("Neither")
        );
    }
}
