pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 输出1~n的全排列中的第k个排列
    // 方法是固定第一个数，计算固定第一个数的全排列有x种，若k<=x，那么结果就在固定的这个数里面，那么计算即可。
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut used = vec![false; n as usize];
        let mut res = String::from("");
        Self::dfs(n, &mut used, &mut res, k);
        res
    }

    fn dfs(mut not_used_num: i32, used: &mut Vec<bool>, res: &mut String, k: i32) -> bool {
        //println!("{},{:?}, {},{}", not_used_num, used, res, k);
        let mut sort_num = 0;
        for i in 0..used.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            not_used_num -= 1;
            let res_num = if not_used_num == 0 {
                0
            } else {
                (1..=not_used_num).fold(1_i32, |times, i| times * i)
            };
            if res_num == 0 && k == 1 {
                res.push(((i as u8 + 1) + b'0') as char);
                return true;
            }
            if res_num >= k - sort_num * res_num {
                // res_num - k + i * res_num
                res.push(((i as u8 + 1) + b'0') as char);
                if Self::dfs(not_used_num, used, res, k - sort_num * res_num) {
                    return true;
                }
                res.pop();
            }
            not_used_num += 1;
            used[i] = false;
            sort_num += 1;
        }
        false
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l060() {
        //println!("i:{},j:{}, res:{}", 5, 1, Solution::get_permutation(5, 1));
        for i in 1..=9 {
            println!("================");
            let nj = (1..=i).fold(1_i32, |times, i| times * i);
            for j in 1..=nj {
                println!("i:{},j:{}, res:{}", i, j, Solution::get_permutation(i, j));
            }
        }
    }
}
