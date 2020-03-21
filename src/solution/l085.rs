pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 给出01组成的矩阵，找出只有1的最大矩阵
    // 其实就是第84题，分别对每行计算当前行的高度，然后调用84题的方法
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut heights = vec![vec![0; n]; m];
        let mut max = std::i32::MIN;
        for i in 0..m {
            for j in 0..n {
                if i == 0 {
                    heights[i][j] = (matrix[i][j] as u8 - b'0') as i32;
                } else {
                    if matrix[i][j] != '0' {
                        heights[i][j] += heights[i - 1][j] as i32 + 1;
                    }
                }
            }
            max = std::cmp::max(max, Self::largest_rectangle_area(heights[i].clone()));
        }
        max
    }

    fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        println!("{:?}", heights);
        if heights.is_empty() {
            return 0;
        }
        let mut stack = vec![(-1, 0)];
        let mut max = std::i32::MIN;
        for (index, height) in heights.iter().enumerate() {
            if stack.is_empty() {
                stack.push((index as i32, *height));
                continue;
            }
            let (_, top_height) = stack[stack.len() - 1];
            if *height < top_height {
                loop {
                    if stack.len() == 1 {
                        break;
                    }
                    let (_, top_height) = stack[stack.len() - 1];
                    let (sec_index, _) = stack[stack.len() - 2];
                    if top_height <= *height {
                        break;
                    }
                    stack.pop().unwrap();
                    max = std::cmp::max(max, (index as i32 - sec_index as i32 - 1) * top_height);
                }
            }
            stack.push((index as i32, *height));
        }
        while stack.len() > 1 {
            let (_, top_height) = stack[stack.len() - 1];
            let (sec_index, _) = stack[stack.len() - 2];

            stack.pop().unwrap();
            max = std::cmp::max(
                max,
                (heights.len() as i32 - sec_index as i32 - 1) * top_height,
            );
        }
        max
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l085() {
        println!(
            "{}",
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ])
        )
    }
}
