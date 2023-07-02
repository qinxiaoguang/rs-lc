use super::Solution;
use super::*;
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    // 找到最大矩形面积
    // 两种思路，一种是枚举宽，去找该宽的最小高
    // 一种是枚举高，去找符合条件的最大宽
    // 其中枚举高找宽的方式，就是找左右两边第一个比自己矮的柱子即可
    // 找第一个比自己矮的柱子可以借助单调栈来解
    // 题目则变为，找到所有元素中左侧第一个比自己小的值的下标,这个题也很经典
    // 如果比栈顶元素大，则栈顶元素就是第一个比自己矮的，如果比栈顶元素小，则出栈，出到当前元素比栈顶元素大为止，再得出结果，并入栈
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut left = vec![-1; heights.len()];
        let mut right = vec![heights.len() as i32; heights.len()];
        let mut stack: Vec<(usize, i32)> = vec![];

        // 遍历第一遍获取left
        for (i, v) in heights.iter().enumerate() {
            // 栈顶元素比自己大的，出栈，直到遇到比自己小的
            while !stack.is_empty() && stack.last().unwrap().1 >= *v {
                // 出栈的时候，说明当前元素的右侧第一个比自己小的就是当前值，所以right也可以更新掉了
                // 注: 这里出栈的时候，对应的pop_idx的left和right的值也已经更新了
                // 所以另一种思路就是出栈的时候把结果计算出来,但是不容易理解和记忆
                let (pop_idx, _) = stack.pop().unwrap();
                right[pop_idx] = i as i32;
            }
            left[i] = if stack.is_empty() {
                -1
            } else {
                // 栈顶元素比自己小
                stack.last().unwrap().0 as i32
            };
            // 当前元素入栈
            stack.push((i, *v));
        }

        let mut ans = 0;
        // 枚举高， 计算结果
        for (i, height) in heights.iter().enumerate() {
            ans = ans.max((right[i] - left[i] - 1) * height);
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_039() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 2]), 4);
        assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 1, 1]), 4);
    }
}
