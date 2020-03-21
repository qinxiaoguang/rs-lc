pub struct Solution {}

macro_rules! v {
    ($fn:expr ;$id2:ident  <- [$start:expr; $end:expr]) => {{
        let mut vec = Vec::new();
        for num in $start..$end {
            vec.push($fn(num));
        }
        vec
    }};
}

impl Solution {
    #[allow(dead_code)]
    // 格雷编码，
    // 每次异或一个值，如 异或 0001 ，并判断异或后的值是否已经被使用过
    // 但是最经典的解法是 res[i]=(i ^ i>> 1), 规律题
    pub fn gray_code(n: i32) -> Vec<i32> {
        v![|x|x ^ x >> 1; x <- [0;2_i32.pow(n as u32)]]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l089() {
        println!("{:?}", Solution::gray_code(4));
    }
}
