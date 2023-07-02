use super::Solution;
use super::*;
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    // 与j2_039一样的， 二维数组，每个元素由01组成，计算由1组成的最大矩形面积
    pub fn maximal_rectangle(matrix: Vec<String>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }
        let mut heights: Vec<i32> = vec![0; matrix[0].len()];
        let mut ans = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let tmp = matrix[i][j..=j].parse::<i32>().unwrap();
                if tmp == 1 {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }
            ans = ans.max(Self::_largest_rectangle_area(heights.clone()));
        }
        return ans;
    }
    pub fn _largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut left = vec![-1; heights.len()];
        let mut right = vec![heights.len() as i32; heights.len()];
        let mut stack: Vec<(usize, i32)> = vec![];

        for (i, v) in heights.iter().enumerate() {
            while !stack.is_empty() && stack.last().unwrap().1 >= *v {
                let (pop_idx, _) = stack.pop().unwrap();
                right[pop_idx] = i as i32;
            }
            left[i] = if stack.is_empty() {
                -1
            } else {
                stack.last().unwrap().0 as i32
            };
            stack.push((i, *v));
        }

        let mut ans = 0;
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
    fn test_j2_040() {
        assert_eq!(Solution::maximal_rectangle(vec![]), 0);
        assert_eq!(Solution::maximal_rectangle(vec!["0".to_string()]), 0);
        assert_eq!(Solution::maximal_rectangle(vec!["1".to_string()]), 1);
        assert_eq!(Solution::maximal_rectangle(vec!["00".to_string()]), 0);
        assert_eq!(
            Solution::maximal_rectangle(vec!["01".to_string(), "10".to_string()]),
            1
        );
    }
}
