use super::Solution;

impl Solution {
    pub fn find_string(words: Vec<String>, s: String) -> i32 {
        // 二分查找，但是需要处理空数组，遇到空数组的时候有两种策略
        // 1. 左右遍历，直到非空数组
        // 2. 左右重新二分
        // 左右重新二分效率好一点
        if words.len() == 0 {
            return -1;
        }
        Self::bineary_find_string(&words, &s, 0, words.len() - 1)
    }

    // 若没找到则为-1
    pub fn bineary_find_string(words: &[String], target: &str, start: usize, end: usize) -> i32 {
        use std::cmp::Ordering::*;
        if start > end {
            return -1;
        }
        let target = target.to_string();
        let mid = (start + end) / 2;
        if words[mid].is_empty() {
            // 如果是空，需要特殊处理
            if mid == 0 {
                return if words[end] == target { end as i32 } else { -1 };
            }
            let left = Self::bineary_find_string(&words, &target, start, mid - 1);
            let right = Self::bineary_find_string(&words, &target, mid + 1, end);
            return match (left, right) {
                (-1, v) | (v, -1) => v,
                _ => -1,
            };
        }
        match words[mid].cmp(&target) {
            Equal => mid as i32,
            Less => Self::bineary_find_string(&words, &target, mid + 1, end),
            Greater if mid != 0 => Self::bineary_find_string(&words, &target, start, mid - 1),
            _ => -1,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_m1005() {
        let res = "YSPBaovrZBS".cmp(&"evMMBOf");
        println!("res = {:?}", res);
        assert_eq!(
            Solution::find_string(
                vec![
                    String::from("at"),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from("ball"),
                    String::from(""),
                    String::from(""),
                    String::from("car"),
                    String::from(""),
                    String::from(""),
                    String::from("dad"),
                    String::from(""),
                    String::from("")
                ],
                String::from("ta")
            ),
            -1
        );
        assert_eq!(
            Solution::find_string(
                vec![
                    String::from("at"),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from("ball"),
                    String::from(""),
                    String::from(""),
                    String::from("car"),
                    String::from(""),
                    String::from(""),
                    String::from("dad"),
                    String::from(""),
                    String::from("")
                ],
                String::from("ball")
            ),
            4
        );
        assert_eq!(
            Solution::find_string(vec![String::from("")], String::from("haha")),
            -1
        );
        assert_eq!(Solution::find_string(vec![], String::from("haha")), -1);
        assert_eq!(
            Solution::find_string(
                vec![
                    String::from("a"),
                    String::from("b"),
                    String::from("c"),
                    String::from("d"),
                    String::from("e"),
                    String::from("f"),
                    String::from("g"),
                ],
                String::from("e")
            ),
            4
        );
        assert_eq!(
            Solution::find_string(
                vec![
                    String::from("DirNnILhARNS hOYIFB"),
                    String::from("SM"),
                    String::from("YSPBaovrZBS"),
                    String::from("evMMBOf"),
                    String::from("mCrS"),
                    String::from("oRJfjw gwuo"),
                    String::from("xOpSEXvfI"),
                ],
                String::from("mCrS")
            ),
            4
        );
    }
}
