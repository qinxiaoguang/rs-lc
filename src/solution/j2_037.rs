use super::Solution;
use super::*;
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    // 碰撞后的行星
    // 利用栈，每次和栈顶的元素比较
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        ans.push(asteroids[0]);

        for &now in asteroids.iter().skip(1) {
            if ans.len() == 0 {
                ans.push(now);
                continue;
            }
            let mut last = *ans.last().unwrap();
            // 左移行星吃掉很多行星的情况
            while ans.len() != 0 && (last > now && (last > 0 && now < 0)) && last.abs() < now.abs()
            {
                ans.pop();
                if ans.len() > 0 {
                    last = *ans.last().unwrap();
                }
            }
            // 两者相互抵消的情况
            if last > now && last == -now {
                ans.pop();
                continue;
            }
            // 相同符号直接push
            if (ans.len() == 0) || (now > 0 && last > 0) || (now < 0 && last < 0) {
                ans.push(now);
                continue;
            }
            // 不同符号，右正左负 直接push
            if now > last {
                ans.push(now);
                continue;
            }

            // 剩下的就是不同符号右负左正的情况,右大左小在前边已经判断过了，所以这里只剩下右小左大的情况，直接continue
        }

        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_037() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
        assert_eq!(
            Solution::asteroid_collision(vec![-2, -2, -2, -2]),
            vec![-2, -2, -2, -2]
        );
        assert_eq!(
            Solution::asteroid_collision(vec![-2, -1, 1, 2]),
            vec![-2, -1, 1, 2]
        );
    }
}
