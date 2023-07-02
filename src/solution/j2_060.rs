use super::*;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    // 出现频率最高的k个数字
    // 使用优先队列，或者叫大顶堆
    // 先使用map来计算数字出现频率
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut res = vec![];
        nums.iter().for_each(|x| {
            *m.entry(x).or_insert(0) += 1;
        });

        for (k, v) in m.iter() {
            heap.push((*v, **k));
        }

        for i in 0..k {
            if heap.is_empty() {
                break;
            }
            res.push(heap.pop().unwrap().1)
        }

        return res;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_060() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        )
    }
}
