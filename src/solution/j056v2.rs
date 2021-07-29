use super::Solution;

impl Solution {
    // 只有一个数出现1次，其他数都出现3次
    // 计算每位上面1出现的次数，最后每位都mod3，就是最终结果
    // 因为除3后，只有3个状态，使用两位二进制就能表示
    // 所以使用i64来表示32位每位对应的状态即可
    // 其中 00 表示 0， 01表示1，10表示2
    // 但写法有点难写，先用数组来表示吧
    pub fn single_number_v2(nums: Vec<i32>) -> i32 {
        let mut state = vec![0; 32];
        nums.iter().for_each(|n| {
            let mut cnt = 0;
            let mut num = *n;
            for i in 0..32 {
                if num >> i & 1 == 1 {
                    state[i] += 1;
                    state[i] %= 3;
                }
            }
        });
        let mut res = 0i32;
        for i in 0..32 {
            if state[i] == 1 {
                res += 2i32.pow(i as u32);
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j056v2() {
        assert_eq!(Solution::single_number_v2(vec![3, 4, 3, 3]), 4);
        assert_eq!(Solution::single_number_v2(vec![9, 1, 7, 9, 7, 9, 7]), 1);
    }
}
