pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 柱状图中能勾勒出的最大面积
    // 此题比较经典，利用栈来进行计算，利用栈来维护一个递增的序列，当遇到一个破坏递增序列的值时，就弹出栈顶元素
    // 利用栈顶元素，来计算包含栈顶元素的组成的最大面积，因为此栈顶元素一定是当前最大值，
    // 所以计算方法就是栈顶元素的高*栈顶元素的栈的上一个坐标的距离
    // 当前栈顶坐标 和当前栈顶次级坐标，中间的所有坐标的高度一定是大于当前栈顶坐标的高度的。
    // 想一想为什么
    // 此题很经典
    //
    // 如果想不出来, 可以循序渐进的来写
    // 1. 首先采用暴力思想, 枚举每个高度, 计算以该高度为最后一个高时, 且包含该高, 其能画出的面积的最大值
    // 那么重点就是找到左边第一个比该高小的值,正常思路是向左遍历
    // 而利用单调栈, 只需要找到次栈顶元素即可.这就是第二个单调栈的一个思想.
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
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
    fn test_l084() {
        println!(
            "{}",
            Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3])
        );

        println!("{}", Solution::largest_rectangle_area(vec![0]));
        println!("{}", Solution::largest_rectangle_area(vec![]));
        println!("{}", Solution::largest_rectangle_area(vec![1]));
        println!("{}", Solution::largest_rectangle_area(vec![2, 1, 2]));
    }
}
