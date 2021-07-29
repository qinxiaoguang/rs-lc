use super::Solution;

impl Solution {
    // 找最小k个数 用大顶堆
    // 找最大k个数 用小顶堆
    // rust里的大顶堆使用 std::collections::BinaryHeap
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = std::collections::BinaryHeap::new();
        let k = k as usize;

        arr.into_iter().for_each(|v| {
            if heap.len() < k {
                heap.push(v);
                return;
            }
            if let Some(top) = heap.peek() {
                if v < *top {
                    heap.pop();
                    heap.push(v);
                }
            }
        });

        let mut res = vec![];
        while let Some(v) = heap.pop() {
            res.push(v);
        }
        res.reverse();
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j040() {
        assert_eq!(
            Solution::get_least_numbers(vec![1, 2, 3, 4, 5], 3),
            vec![1, 2, 3]
        );

        assert_eq!(Solution::get_least_numbers(vec![1, 2], 3), vec![1, 2]);
    }
}
