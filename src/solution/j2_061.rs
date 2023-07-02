use super::*;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    // 给定两个以升序排列的整数数组 nums1 和 nums2 , 以及一个整数 k 。
    // 定义一对值 (u,v)，其中第一个元素来自 nums1，第二个元素来自 nums2 。
    // 请找到和最小的 k 个数对 (u1,v1),  (u2,v2)  ...  (uk,vk) 。
    // 只需要从两个数的前k个数中进行组合就可以了
    // 但这样复杂度是k^2 * logk
    // 优化的方式就是使用大顶堆的思路来进行解，一开始就固定数组2中的第一个数，从数组1中找出k个数，将结果全push到小顶堆中
    // 接着开始固定第2个数，来进行push，但是如果固定第2个数的时候，一旦发现两者的和大于小顶堆的最大值时，就直接开启下一个循环
    // 这样的话， 遍历的时间复杂度基本上只有k了
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let nums1len = nums1.len().min(k as usize);
        let nums2len = nums2.len().min(k as usize);
        for i in 0..nums1len {
            for j in 0..nums2len {
                let sum = nums1[i] + nums2[j];
                if heap.len() != k as usize {
                    heap.push((sum, nums1[i], nums2[j]));
                    continue;
                }

                // 开始获取大顶堆的最大值
                if heap.peek().unwrap().0 <= sum {
                    break;
                }
                heap.pop();
                heap.push((sum, nums1[i], nums2[j]));
            }
        }

        let mut ans = vec![];
        while let Some((_, x, y)) = heap.pop() {
            ans.push(vec![x, y]);
        }
        ans.reverse();

        return ans;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_061() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![[1, 1], [1, 1]]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
            vec![[1, 3], [2, 3]]
        );
    }
}
