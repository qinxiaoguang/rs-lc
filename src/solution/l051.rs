pub struct Solution {}
impl Solution {
    // 经典 n皇后问题
    // 利用col,rom以及bias来保存当前的某列某行某斜对焦是否有对应的皇后出现。
    // 其中左斜对角的规律是 i-j 相同，右斜对角的规律是i+j相等
    #[allow(dead_code)]
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n <= 0 {
            return vec![];
        }
        let len = n as usize;
        let (mut col, mut row) = (vec![false; len], vec![false; len]);
        let (mut left_bias, mut right_bias) = (vec![false; 2 * len - 1], vec![false; 2 * len - 1]);
        let mut res = vec![];
        let mut tmp = vec![vec!['.'; len].iter().collect::<String>(); len];
        Self::dfs(
            0_i32,
            n,
            &mut col,
            &mut row,
            &mut left_bias,
            &mut right_bias,
            &mut res,
            &mut tmp,
        );
        res
    }

    fn dfs(
        deep: i32,
        n: i32,
        col: &mut [bool],
        row: &mut [bool],
        left_bias: &mut [bool],
        right_bias: &mut [bool],
        res: &mut Vec<Vec<String>>,
        tmp: &mut Vec<String>,
    ) {
        if deep == n {
            res.push(tmp.clone());
        }
        let i = deep;
        for j in 0..n {
            let left = (j - i + n - 1) as usize;
            if col[j as usize] || row[i as usize] || left_bias[left] || right_bias[(j + i) as usize]
            {
                continue;
            }
            // 未被使用
            col[j as usize] = true;
            row[i as usize] = true;
            left_bias[left] = true;
            right_bias[(j + i) as usize] = true;
            unsafe {
                tmp[i as usize].as_bytes_mut()[j as usize] = b'Q';
            }
            Self::dfs(deep + 1, n, col, row, left_bias, right_bias, res, tmp);
            unsafe {
                tmp[i as usize].as_bytes_mut()[j as usize] = b'.';
            }
            col[j as usize] = false;
            row[i as usize] = false;
            left_bias[left] = false;
            right_bias[(j + i) as usize] = false;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l051() {
        println!("{:?}", Solution::solve_n_queens(0));
        println!("{:?}", Solution::solve_n_queens(1));
        println!("{:?}", Solution::solve_n_queens(2));
        println!("{:?}", Solution::solve_n_queens(3));
        println!("{:?}", Solution::solve_n_queens(4));
    }
}
