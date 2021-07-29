use super::Solution;

impl Solution {
    // 记忆化搜索
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = graph.len();
        let mut rem = std::collections::HashMap::new();
        if n == 0 {
            return vec![];
        }
        let mut used = vec![false; n];
        Self::l787_dfs(&graph, 0, &mut used, &mut rem)
    }

    fn l787_dfs(
        graph: &Vec<Vec<i32>>,
        i: usize,
        used: &mut Vec<bool>,
        rem: &mut std::collections::HashMap<usize, Vec<Vec<i32>>>,
    ) -> Vec<Vec<i32>> {
        if i == graph.len() - 1 {
            return vec![vec![i as i32]];
        }
        if rem.contains_key(&i) {
            return rem.get(&i).unwrap().clone();
        }
        used[i] = true;
        let mut res = vec![];

        for &j in graph[i].iter() {
            if !used[j as usize] {
                let tmp_path = Self::l787_dfs(graph, j as usize, used, rem);
                tmp_path.into_iter().for_each(|mut v| {
                    v.insert(0, i as i32);
                    res.push(v);
                });
            }
        }

        used[i] = false;
        rem.insert(i, res.clone());
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l797() {
        let res = Solution::all_paths_source_target(matrix![[1, 2], [3], [3], []]);
        println!("res = {:?}", res);

        let res = Solution::all_paths_source_target(matrix![[4, 3, 1], [3, 2, 4], [3], [4], []]);
        println!("res = {:?}", res);

        let res = Solution::all_paths_source_target(matrix![[1], []]);
        println!("res = {:?}", res);

        let res = Solution::all_paths_source_target(matrix![[1, 2, 3], [2], [3], []]);
        println!("res = {:?}", res);

        let res = Solution::all_paths_source_target(matrix![[1, 3], [2], [3], []]);
        println!("res = {:?}", res);

        let res = Solution::all_paths_source_target(matrix![]);
        println!("res = {:?}", res);
    }
}
