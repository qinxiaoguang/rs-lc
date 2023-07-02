use super::Solution;

impl Solution {
    // 计算有多少个回文子串
    // 中心扩散法可解,需要在中间添加'#'分割符
    // 再简单的方法就是使用马拉车算法
    // 马拉车其实也是中心扩散法，只不过中心扩散是一步步的找，
    // 而马拉车则是利用辅助数组直接找
    // 马拉车算法是求最长回文子串的，辅助数组记录的是每一位的最长回文半径
    // 最终的结果就是将这些最长值求和就得到了结果，最终的时间复杂度是O(n)
    // 马拉车的算法要点是需要记录当前遍历的maxRight及对应的center
    // 利用maxRight,center的中心对称 以及与i点的关系，去查找当前i点的最大半径
    // 基本上分为几种情况
    // 1. maxRight <= i: 此时需要使用原始的中心扩散
    // 2. maxRight > i: 只有两种情况,
    //      一种是获取到对称点，且结果大于maxRight，则继续中心扩散,
    //      另一种是获取到对称点，结果未超过maxRight,则为最终结果,这种结果为p[2*center-i]，其中2*center-i即为i关于center的对称点
    // 这两种描述为语言:   p[i] = p[2center - i].min(maxRight - i) 只需要记住这个即可
    // 马拉车写法非常简单，只需要这几步:
    // 1. 预处理
    // 2. 计算p值,公式: p[i] = max_right <=i ? 1 : p[2*center - i].min(max_right - i);
    // 3. 中心扩散，上述所有场景都可以复用这个中心扩散
    // 4. 更新max_right,center
    pub fn count_substrings(s: String) -> i32 {
        // s预处理
        if s.len() <= 1 {
            return s.len() as i32;
        }
        let mut s: Vec<char> = s
            .chars()
            .map(|c| "#".to_string() + &c.to_string())
            .collect::<Vec<String>>()
            .concat()
            .chars()
            .collect();
        s.push('#');
        // 回文半径，默认为1
        let mut p = vec![0; s.len()];
        p[0] = 1;
        let mut ans = 0;

        let (mut center, mut max_right) = (0usize, 0usize);
        for i in 1..s.len() {
            p[i] = if max_right <= i {
                1
            } else {
                // 为什么是两者最小值，因为如果max_right-i比p[2*center-i]小的话，说明至少对于max_right-i来说一定是回文串内的。
                p[2 * center - i].min(max_right - i)
            };

            // 中心扩散
            while i >= p[i] && i + p[i] < s.len() && s[i - p[i]] == s[i + p[i]] {
                p[i] += 1;
            }

            // 更新max_right
            if i + p[i] > max_right {
                max_right = i + p[i];
                center = i;
            }
            ans += p[i] / 2;
        }

        return ans as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_020() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
        assert_eq!(Solution::count_substrings("a".to_string()), 1);
        assert_eq!(Solution::count_substrings("aa".to_string()), 3);
    }
}
