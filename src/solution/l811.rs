pub struct Solution {}
/*
 * @lc app=leetcode.cn id=811 lang=rust
 *
 * [811] 子域名访问计数
 *
 * https://leetcode-cn.com/problems/subdomain-visit-count/description/
 *
 * algorithms
 * Easy (68.28%)
 * Likes:    74
 * Dislikes: 0
 * Total Accepted:    10.5K
 * Total Submissions: 15.3K
 * Testcase Example:  '["9001 discuss.leetcode.com"]'
 *
 *
 * 一个网站域名，如"discuss.leetcode.com"，包含了多个子域名。作为顶级域名，常用的有"com"，下一级则有"leetcode.com"，最低的一级为"discuss.leetcode.com"。当我们访问域名"discuss.leetcode.com"时，也同时访问了其父域名"leetcode.com"以及顶级域名
 * "com"。
 *
 * 给定一个带访问次数和域名的组合，要求分别计算每个域名被访问的次数。其格式为访问次数+空格+地址，例如："9001
 * discuss.leetcode.com"。
 *
 * 接下来会给出一组访问次数和域名组合的列表cpdomains 。要求解析出所有域名的访问次数，输出格式和输入格式相同，不限定先后顺序。
 *
 *
 * 示例 1:
 * 输入:
 * ["9001 discuss.leetcode.com"]
 * 输出:
 * ["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
 * 说明:
 *
 * 例子中仅包含一个网站域名："discuss.leetcode.com"。按照前文假设，子域名"leetcode.com"和"com"都会被访问，所以它们都被访问了9001次。
 *
 *
 *
 * 示例 2
 * 输入:
 * ["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"]
 * 输出:
 * ["901 mail.com","50 yahoo.com","900 google.mail.com","5 wiki.org","5 org","1
 * intel.mail.com","951 com"]
 * 说明:
 * 按照假设，会访问"google.mail.com" 900次，"yahoo.com" 50次，"intel.mail.com"
 * 1次，"wiki.org" 5次。
 * 而对于父域名，会访问"mail.com" 900+1 = 901次，"com" 900 + 50 + 1 = 951次，和 "org" 5 次。
 *
 *
 * 注意事项：
 *
 *
 * cpdomains 的长度小于 100。
 * 每个域名的长度小于100。
 * 每个域名地址包含一个或两个"."符号。
 * 输入中任意一个域名的访问次数都小于10000。
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut m = HashMap::new();
        for s in cpdomains {
            let ss: Vec<&str> = s.split(" ").collect();
            let times: i32 = ss[0].parse().unwrap();
            let url = ss[1];
            let domain_items: Vec<&str> = url.split(".").collect();
            for i in 0..domain_items.len() {
                let domain = domain_items[i..].join(".");
                *m.entry(domain).or_insert(0) += times;
            }
        }
        let mut res = vec![];
        for (k, v) in m {
            res.push(format!("{} {}", v, k));
        }
        res
    }
}
// @lc code=end
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l811() {
        println!(
            "{:?}",
            Solution::subdomain_visits(vec![
                String::from("900 google.mail.com"),
                String::from("50 yahoo.com"),
                String::from("1 intel.mail.com"),
                String::from("5 wiki.org")
            ])
        );
        println!("{:?}", Solution::subdomain_visits(vec![]));
    }
}
