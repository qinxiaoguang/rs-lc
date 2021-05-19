use super::Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        // 可以用贪心
        // 每次从可用字符中找出最多的
        let mut res = String::from("");
        let mut can_choose = [true, true, true];
        let mut choose_cnt = [a, b, c];
        let mut last_cnt = 0;
        let mut last_char: i32 = -1;
        while choose_cnt.iter().any(|&cnt| cnt > 0) && can_choose.iter().any(|&c| c) {
            // 找出最多的字母
            let mut choose_set = choose_cnt
                .iter()
                .zip(&can_choose)
                .map(|(&cnt, &can_choose)| if can_choose && cnt > 0 { cnt } else { -1 })
                .collect::<Vec<i32>>();
            let next_choose = choose_set
                .iter()
                .enumerate()
                .max_by_key(|a| a.1)
                .map(|(idx, &v)| if v != -1 { idx as i32 } else { -1 })
                .unwrap();

            match next_choose {
                0 => res.push('a'),
                1 => res.push('b'),
                2 => res.push('c'),
                _ => break,
            }
            choose_cnt[next_choose as usize] -= 1;
            if last_char == next_choose as i32 {
                last_cnt += 1;
                if last_cnt >= 2 {
                    can_choose[last_char as usize] = false;
                }
            } else {
                last_char = next_choose as i32;
                last_cnt = 1;
                can_choose[0] = true;
                can_choose[1] = true;
                can_choose[2] = true;
            }
        }

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1405() {
        println!(
            "Solution::longest_diverse_string(7,2,1) = {:?}",
            Solution::longest_diverse_string(7, 2, 1)
        );

        println!(
            "Solution::longest_diverse_string(2,2,1) = {:?}",
            Solution::longest_diverse_string(2, 2, 1)
        );

        println!("Solution = {:?}", Solution::longest_diverse_string(7, 1, 0));
        println!(
            "Solution::longest_diverse_string(0,0,0) = {:?}",
            Solution::longest_diverse_string(0, 0, 0)
        );
    }
}
