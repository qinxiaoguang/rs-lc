use super::Solution;

impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        // 先放0， 0左侧及右侧，都可以随便分配及排列
        // 所以优先找0适合的位置，0左侧有待分配的i个数，右侧有待分配的j个数，f(i)表示i个数的排列个数
        // 那么res = C(i+j, i)*f(i) * f(j), C是组合数
        // 0的位置只能在DI上,或者开头的I及末尾的D上
        // 需要使用记忆化搜索进行优化
        // 建议 ： 与MOD有关的都使用i64
        let MOD = 1000000007;
        let c = Self::cal_c(s.len() + 1, s.len() + 1, MOD);
        let mut rem = std::collections::HashMap::new();
        Self::l903_f(&s, &c, &mut rem) as i32
    }

    // 计算组合数
    // 递推公式 C[n][m] = c[n-1][m-1] + c[n-1][m]
    fn cal_c(n: usize, m: usize, MOD: i64) -> Vec<Vec<i64>> {
        let mut c = vec![vec![0; m]; n];
        c[1][0] = 1;
        c[1][1] = 1;
        for i in 2..n {
            c[i][0] = 1;
            for j in 1..m {
                c[i][j] = c[i - 1][j - 1] % MOD + c[i - 1][j] % MOD;
            }
        }
        c
    }

    // 计算n个数的排列数
    fn l903_f(s: &str, C: &[Vec<i64>], rem: &mut std::collections::HashMap<String, i64>) -> i64 {
        let MOD = 1000000007;
        if rem.contains_key(s) {
            return *rem.get(s).unwrap();
        }
        let mut l = s.len();
        if l == 0 || l == 1 {
            return 1;
        }

        let mut ans = 0i64;
        // 查找0符合的位置
        let chars: Vec<char> = s.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            if i == 0 {
                if c == 'I' {
                    // 符合要求
                    ans += Self::l903_f(&s[1..], C, rem);
                    ans %= MOD;
                }
                continue;
            }
            if i == s.len() - 1 && c == 'D' {
                ans += Self::l903_f(&s[..s.len() - 1], C, rem);
                ans %= MOD;
                continue;
            }
            if chars[i - 1] == 'D' && chars[i] == 'I' {
                ans += ((((Self::l903_f(&s[i + 1..], C, rem)
                    * Self::l903_f(&s[..i - 1], C, rem))
                    % MOD)
                    * (C[s.len()][i] % MOD))
                    % MOD);
            }
        }
        rem.insert(s.to_string(), (ans % MOD));
        (ans % MOD)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l903() {
        let ans = 1000000007i64 * 1000000007i64;
        println!("ans = {:?}", ans);
        assert_eq!(Solution::num_perms_di_sequence(s!("DID")), 5);
        assert_eq!(Solution::num_perms_di_sequence(s!("DI")), 2);
        assert_eq!(Solution::num_perms_di_sequence(s!("ID")), 2);
        assert_eq!(Solution::num_perms_di_sequence(s!("DDDD")), 1);
        assert_eq!(Solution::num_perms_di_sequence(s!("IDI")), 5);
        assert_eq!(
            Solution::num_perms_di_sequence(s!("IDDDIIDIIIIIIIIDIDID")),
            853197538
        );

        assert_eq!(
            Solution::num_perms_di_sequence(s!(
                "IIDIIDDIDDDDIIDDIDIDIDDDDIDDDIIIIDDIDDDDIDIIDDIDID"
            )),
            997381513
        );

        assert_eq!(Solution::num_perms_di_sequence(s!("IIDDIDDIIDDIDIDDIDDDDIIIDIDIDDDDDIIDIDDIDIIDIDDIIIDIIIIIIIIDIDIDIDDIDIIIIDDIIIIDDDDIIIDDIIDDIDIIIDDDDDDIIDIDDIIDDDDIIDDDIDIDDDIIIDIDIDIIIDDIDDDDDDDDIIDDIDDDIDDDIDDDDIIDIIIDDIDDDIDDIDDDIIDDIIIDIDIIDIDI")),642107237);
    }
}
