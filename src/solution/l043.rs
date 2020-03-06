pub struct Solution {}
// String::from == sf!
macro_rules! sf {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    // 计算两个数相乘，不能用大数库
    #![allow(dead_code)]
    pub fn multiply(num1: String, num2: String) -> String {
        if num1.is_empty() || num2.is_empty() {
            return sf!("");
        }
        let mut zeros = sf!("");
        let (num1_bytes, num2_bytes) = (num1.as_bytes(), num2.as_bytes());
        let mut result = sf!("");
        for i in (0..num2.len()).rev() {
            let mut over_num = 0;
            let mut res = sf!("");
            let times = num2_bytes[i] - '0' as u8;
            for j in (0..num1.len()).rev() {
                let left = num1_bytes[j] - '0' as u8;
                let sum = left * times + over_num;
                over_num = sum / 10;
                res.push((sum % 10 + '0' as u8) as char);
            }
            if over_num != 0 {
                res.push((over_num + '0' as u8) as char);
            }
            res = res.chars().rev().collect();
            res.push_str(&zeros);
            result = Self::bignum_add(res, result);
            zeros.push('0');
        }
        let mut first_zero = true;
        result = result
            .chars()
            .filter(|c| {
                if first_zero && c == &'0' {
                    return false;
                }
                first_zero = false;
                return true;
            })
            .collect();
        if result.is_empty() {
            return sf!("0");
        }
        return result;
    }

    fn bignum_add(num1: String, num2: String) -> String {
        let (len1, len2) = (num1.len(), num2.len());
        let mut num1 = num1;
        let mut num2 = num2;
        let add_zero_len = (len2 as i32 - len1 as i32).abs();
        let mut add_zero = sf!("");
        (0..add_zero_len).for_each(|_| {
            add_zero.push('0');
        });
        if len1 < len2 {
            num1 = format!("{}{}", add_zero, num1);
        } else {
            num2 = format!("{}{}", add_zero, num2);
        }
        let mut res = sf!("");
        let nums1_bytes = num1.as_bytes();
        let nums2_bytes = num2.as_bytes();
        let mut over_flow = false;
        for i in (0..num1.len()).rev() {
            let (n1, n2) = (nums1_bytes[i], nums2_bytes[i]);
            let sum = (n1 - '0' as u8) + (n2 - '0' as u8) + if over_flow { 1 } else { 0 };
            let sum = if sum >= 10 {
                over_flow = true;
                sum - 10
            } else {
                over_flow = false;
                sum
            };
            res.push((sum + '0' as u8) as char);
        }
        if over_flow {
            res.push('1');
        }
        res.chars().rev().collect()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l043() {
        assert_eq!("89176", Solution::bignum_add(sf!("81284"), sf!("7892")));
        assert_eq!("641493328", Solution::multiply(sf!("81284"), sf!("7892")));
        assert_eq!("0", Solution::multiply(sf!("81284"), sf!("0")));
        assert_eq!("6", Solution::multiply(sf!("2"), sf!("3")));
        assert_eq!("56088", Solution::multiply(sf!("123"), sf!("456")));
        println!(
            "{}",
            Solution::multiply(
                sf!("198023479801237490812734091798172348971293874981"),
                sf!("289477148923741972341234123891236749172938479812374987192346791263578165286185612346182734")
            )
        );
    }
}
