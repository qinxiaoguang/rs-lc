use super::Solution;
use std::collections::VecDeque;

impl Solution {
    // 求滑动窗口中，每个个窗口里的最大值
    // 首先 采用大顶堆来进行解题，但是大顶堆有个问题，就是最大的值，不一定是当前滑动窗口中的值
    // 所以需要在该堆中保存其下标，当弹出栈顶元素，其下标不在当前窗口内，则继续弹
    // 这种方法的时间复杂度为O(nlogk)
    // 而优化方法，则是采用单调队列
    // 队列中的元素类比大顶堆，是一个递减的队列
    // 滑动窗口在滑动的时候，要往里添加一个元素，同时删掉一个元素
    // 删元素的时候，只需要比较下标即可，而添加元素的时候，需要比较队列末尾元素和当前要添加元素的大小
    // 如果比末尾的元素大，则弹出末尾元素，继续比较，直到末尾元素比当前元素大，则将当前元素压入
    // 这样就模拟了优先队列(不明觉历)
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut q: VecDeque<(usize, i32)> = std::collections::VecDeque::new();
        let mut res = Vec::new();
        // 先把k个元素搞到队列中，
        nums.iter().enumerate().for_each(|(idx, &n)| {
            // 弹出队前元素
            if idx >= k as usize {
                if let Some(v) = q.front() {
                    if v.0 == idx - k as usize {
                        q.pop_front();
                    }
                }
            }
            // 插入队尾元素
            while let Some(v) = q.back() {
                if v.1 <= n {
                    q.pop_back();
                } else {
                    q.push_back((idx, n));
                    break;
                }
            }
            if q.len() == 0 {
                q.push_back((idx, n));
            }
            // 获取最大值
            if idx >= k as usize - 1 {
                res.push(q.front().unwrap().1)
            }
        });

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j059v1() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}
