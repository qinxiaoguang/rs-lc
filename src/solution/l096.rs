pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 卡特兰数
    // 其应用一般有: 1. 入栈出栈的方式的个数 2.合法左右括号组成的括号种类数 3. 此题，生成二插搜索树
    // C(n) = 1/n+1 * C(n,2n)
    // C(0) = C(1) = 1
    // C(n) = C(0)*C(n-1) + C(1)*C(n-2) + .. + C(n-1)*C(0)
    // 解释上上边的公式是怎么来的，以左右括号为题，那么C(0)*C(n-1)的意思就是
    // 选第1个位置为合法左右括号，那么其左侧合法的括号个数为C(0)，右侧剩余n-1对括号，组成合法括号为C(n-1),
    // 那么总的合法个数为C(0)*C(n-1),一定要注意n是括号的对数
    // C(n) = C(n-1) * (4^n-2)/(n+1)
    pub fn num_trees(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut c = vec![0_i32; n as usize + 1];
        c[0] = 1;
        c[1] = 1;
        for i in 2..=n as usize {
            for j in 0..i {
                c[i] += c[j] * c[i - j - 1];
            }
        }
        c[n as usize]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l096() {
        for i in 0..10 {
            println!("{}", Solution::num_trees(i));
        }
    }
}
