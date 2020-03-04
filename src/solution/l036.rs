pub struct Solution {}
use std::collections::{HashMap, HashSet};
// String::from == sf!
macro_rules! sf {
    ($str:expr) => {
        String::from($str)
    };
}

// map macro
macro_rules! map {
    ($($k:expr => $v:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
}

// set macro
macro_rules! set {
    ($($k:expr),*) => {
        {
            let mut set = HashSet::new();
            $(
                map.insert($k);
            )*
            set
        }
    };
}

impl Solution {
    // 判断一个数独是否合法，数独是9*9的，空白字符为'.'
    // 思路很简单，就是遍历一遍，如board[i][j]的数为7，那么此i行j列不允许再有7
    // 同时此9*9方格也不准有7，那么问题是怎么准确的找到9*9方格的位置，其实就是[i/3][j/3]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut col_set: Vec<HashSet<char>> = vec![set![]; 9];
        let mut row_set: Vec<HashSet<char>> = vec![set![]; 9];
        let mut square: Vec<Vec<HashSet<char>>> = vec![vec![set![]; 3]; 3];
        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];

                if (c == '.')
                    || (Self::contains_or_insert(&mut col_set[i], c)
                        && Self::contains_or_insert(&mut row_set[j], c)
                        && Self::contains_or_insert(&mut square[i / 3][j / 3], c))
                {
                    continue;
                }
                return false;
            }
        }
        true
    }

    // true表示插入成功
    fn contains_or_insert(set: &mut HashSet<char>, c: char) -> bool {
        if set.contains(&c) {
            return false;
        }
        set.insert(c);
        true
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l036() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        println!("{}", Solution::is_valid_sudoku(board));
    }
}
