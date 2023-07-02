use super::Solution;
use super::*;
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    // 给一个每日气温表，求每天想要观测的更高的气温，需要等待几天时间
    // 升序就更新，降序就入栈
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![0; temperatures.len()];
        stack.push((0, temperatures[0]));
        temperatures.iter().enumerate().skip(1).for_each(|(i, x)| {
            if ans.len() == 0 {
                stack.push((i, *x));
                return;
            }
            while !stack.is_empty() {
                let last = stack.last().unwrap();
                if *x > last.1 {
                    // 更新
                    ans[last.0] = (i - last.0) as i32;
                    stack.pop();
                    continue;
                }

                break;
            }
            stack.push((i, *x));
        });
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_038() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );

        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
