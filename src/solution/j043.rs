use super::Solution;

impl Solution {
    // 按位来依次进行计算
    // 先计算个位数上1出现的次数
    // 再计算十位数上1出现的次数
    // 依次类推
    // 计算方式也有规律,假设当前位是1，则将当前位的1去掉，剩下的值的范围就是1出现的个数
    // 如 000~213 会出现 00~23等次数，即24次 ，所以结果为 2*10 + 3 + 1
    // 剩下两种情况，分别是当前位是0和当前位是2~9的情况，直接看答案分析吧
    // cur -> 1 => high * digt + low + 1
    // cur -> 0 => high * digt
    // cur -> _ => (high+1)*digt
    pub fn count_digit_one(n: i32) -> i32 {
        // 不想写了
        0
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j043() {}
}
