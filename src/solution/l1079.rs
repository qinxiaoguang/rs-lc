use super::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        // 非空且不能重复,dfs + 剪枝
        let mut cache = HashSet::new();
        let mut res = 0;
        let tiles: Vec<char> = tiles.chars().collect();
        for i in 0..tiles.len() {
            Self::dfs(
                &tiles,
                &mut vec![false; tiles.len()],
                i,
                &mut cache,
                String::from(""),
                &mut res,
            );
        }
        res
    }

    fn dfs(
        titles: &[char],
        used: &mut Vec<bool>,
        next: usize,
        cache: &mut HashSet<String>,
        s: String,
        res: &mut i32,
    ) {
        used[next] = true;
        let mut next_s = s.clone();
        next_s.push(titles[next]);
        if cache.contains(&next_s) {
            used[next] = false;
            return;
        }
        *res += 1;
        //println!("{:?}", next_s);
        cache.insert(next_s.clone());
        for i in 0..titles.len() {
            if used[i] {
                continue;
            }
            Self::dfs(titles, used, i, cache, next_s.clone(), res);
        }
        used[next] = false;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1079() {
        assert_eq!(Solution::num_tile_possibilities(String::from("AAB")), 8);
        assert_eq!(
            Solution::num_tile_possibilities(String::from("AAABBC")),
            188
        );
    }
}
