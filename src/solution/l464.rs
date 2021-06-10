use super::Solution;
use std::cmp::max;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        // 判断是否能赢，只需要进行dfs，若自己可以赢，则挑选了一个数，使得下家挑选所有数都不能赢，按这个逻辑去写即可
        // 其中,dfs(max_choosable, desired_total, state, reme) 表示当前状态能否赢, reme表示记忆化搜索的结果 。
        if (1 + max_choosable_integer) * max_choosable_integer / 2 < desired_total {
            return false;
        }
        if max_choosable_integer >= desired_total {
            return true;
        }
        Self::l464_dfs(
            max_choosable_integer,
            desired_total,
            0,
            &mut vec![vec![0; 1 << max_choosable_integer + 1]; desired_total as usize],
        )
    }

    fn l464_dfs(
        max_choosable: i32,
        desired_total: i32,
        state: i32,
        reme: &mut Vec<Vec<u8>>,
    ) -> bool {
        if desired_total <= 0 {
            // 上家已赢
            return false;
        }
        // 挑一个下家无论选什么都赢不了的数字
        // 如果挑的数字下家可以赢，则跳过
        let mut state_copy = state;
        let mut choose_num = 0;
        while choose_num < max_choosable {
            choose_num += 1;
            state_copy = state_copy >> 1;
            if state_copy & 1 != 1 {
                // 此数没被选
                let next_state = state | (1 << (choose_num));
                let next_desired_total = desired_total - choose_num;
                if next_desired_total > 0 {
                    match reme[next_desired_total as usize][next_state as usize] {
                        // 3表示下家可以赢,
                        3 => continue,
                        // 4 表示下家怎么都赢不了
                        4 => {
                            return true;
                        }
                        _ => {}
                    };
                }
                let can_win = Self::l464_dfs(max_choosable, next_desired_total, next_state, reme);
                if next_desired_total > 0 {
                    reme[next_desired_total as usize][next_state as usize] =
                        if can_win { 3 } else { 4 };
                }
                if !can_win {
                    // choose_num这个数，导至下家选什么都赢不了
                    return true;
                }
            }
        }

        // 挑不出来一个数，使得下家不能赢
        false
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l464() {
        assert_eq!(Solution::can_i_win(10, 11), false);
        assert_eq!(Solution::can_i_win(19, 190), true);
    }
}
