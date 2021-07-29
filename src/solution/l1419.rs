use super::Solution;

impl Solution {
    // 模拟青娃，遍历一遍后，查看当前所有的青蛙是否都叫完
    // 如果在模拟的时候，青娃不够用，就添加一个青蛙
    // 使用map来进行模拟,map[a] = 2就表示有两只青蛙叫到a处了
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut pre = [-1; 26];
        //pre[]
        pre[2] = -2; // 初始
        pre[17] = 2; // r -> c
        pre[14] = 17; // o -> r
        pre[0] = 14; // a -> o
        pre[10] = 0; // k -> a
        let mut m = std::collections::HashMap::new();

        let cs = croak_of_frogs.chars().map(|c| c as i32 - 'a' as i32);
        let mut res = 0;
        for n in cs {
            if pre[n as usize] == -2 {
                *m.entry(n).or_insert(0) += 1;
                // 有一只娃已经叫完了，所以停留在最后一处,在最后一处借一只娃
                let mut k = m.entry(10).or_insert(0i32);
                *k -= 1;
                // 最多借了几只蛙
                res = res.max((*k).abs());
            } else {
                let pre = pre[n as usize];
                let pre_cnt = m.entry(pre).or_insert(0);
                if *pre_cnt <= 0 {
                    return -1;
                }
                // 去掉正在此初叫的蛙
                *pre_cnt -= 1;
                *m.entry(n).or_insert(0) += 1;
            }
        }
        for (k, v) in m {
            if v != 0 {
                return -1;
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1419() {
        assert_eq!(Solution::min_number_of_frogs(s!("croakcroak")), 1);
        assert_eq!(Solution::min_number_of_frogs(s!("croakcroakc")), -1);
        assert_eq!(Solution::min_number_of_frogs(s!("crcoakroak")), 2);
    }
}
