use super::Solution;
use std::cmp::Ordering::{Equal, Less};

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        // O(n)复杂度，dp问题，最主要的是条件转移方程是什么
        // 首先还是得用滑动窗口，计算出每个以i为下标开头的符合条件的长度数组
        // 如[1,2,3] ,target=3, 转换为[2,0,1]
        // 那么答案变为，找到两个不为0的数，且其中两个数的距离大于等于第一个数
        // 那么此时可以使用dp来解
        // 设arr转换后的数组为f,那么f[j]就表示以j为开头的符合条件的子串长度
        // 设r[i]为(i, len)中符合条件的子串的最小值
        // 设dp[i]为(i,len)中符合条件的两个子串和的最小值，初始化dp[len] = len+1表示无解
        // 则，dp[j] = min(dp[j+1], f[j] + r[j+f[j]]),
        let (mut left, mut right) = (0usize, 0usize);
        let mut f = vec![0i32; arr.len()];
        let mut sum = arr[0];
        while right < arr.len() {
            if sum < target {
                right += 1;
                if right == arr.len() {
                    break;
                }
                sum += arr[right];
            } else {
                if sum == target {
                    f[left] = (right - left + 1) as i32;
                }
                sum -= arr[left];
                left += 1;
            }
        }
        let mut r = vec![f.len() as i32 + 1; f.len() + 1];
        for i in (0..f.len()).rev() {
            r[i] = if f[i] != 0 {
                std::cmp::min(r[i + 1], f[i])
            } else {
                r[i + 1]
            }
        }

        let mut dp = vec![(f.len() + 1) as i32; f.len() + 1];

        for j in (0..f.len()).rev() {
            dp[j] = if f[j] == 0 {
                dp[j + 1]
            } else {
                std::cmp::min(dp[j + 1], f[j] + r[j + f[j] as usize])
            }
        }
        if dp[0] as usize == f.len() + 1 {
            -1
        } else {
            dp[0]
        }
    }

    // 最开始想的方法，O(nlogn)的复杂度，结果超时,只能去思考O(n)复杂度的解法了
    pub fn min_sum_of_lengths_2(arr: Vec<i32>, target: i32) -> i32 {
        // 首先使用滑动窗口，将数组转换为以当前下标为初始数符合条件的元素长度
        // 如 [1,2,3] target=3, 则数组变为[2,0, 1]
        // 那么问题就转变为，找出数组中两个数相加且距离不小于第一个数的最小值
        let (mut left, mut right) = (0usize, 0usize);
        let mut arr2 = vec![0i32; arr.len()];
        let mut sum = arr[0];
        while right < arr.len() {
            if sum < target {
                right += 1;
                if right == arr.len() {
                    break;
                }
                sum += arr[right];
            } else {
                if sum == target {
                    arr2[left] = (right - left + 1) as i32;
                }
                sum -= arr[left];
                left += 1;
            }
        }

        // 从arr2中找出两个数 (i1, v1), (i2,v2), 其中i是下标，v是值
        // 满足 v1 + v2最小，且i2-i1 >= v1
        // 为了简化，直接将其变为(i,v)数组
        let mut arr: Vec<(usize, i32)> = arr2
            .into_iter()
            .enumerate()
            .filter(|(i, v)| v > &0)
            .collect();

        // 排序
        arr.sort_unstable_by(|i1, i2| {
            let ord = i1.1.cmp(&i2.1);
            if ord == Equal {
                i1.0.cmp(&i2.0)
            } else {
                ord
            }
        });
        //println!("arr3 = {:?}", arr3);
        let mut min = i32::MAX;
        if arr.len() <= 1 {
            return -1;
        }

        // 查找最小值,排序后，先固定left,再找到right符合条件的，并剪枝，但是剪得有点复杂
        let mut find = false;
        let mut left_idx_min = arr.len() - 1;
        let (mut left, mut right) = (0usize, 1usize);
        while left < left_idx_min && (arr[left].1 as i32) < min {
            if right == arr.len() {
                left += 1;
                right = left + 1;
                continue;
            }

            let (sidx, bidx) = if arr[right].0 < arr[left].0 {
                (arr[right], arr[left])
            } else {
                (arr[left], arr[right])
            };
            let sum = sidx.1 + bidx.1;
            if sidx.0 as i32 + sidx.1 <= bidx.0 as i32 {
                if min > sum {
                    min = sum;
                    find = true;
                    left_idx_min = right;
                }
                left += 1;
                right = left + 1;
            } else {
                // 不符合条件，继续遍历right
                right += 1;
            }
        }

        if find {
            min
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1477() {
        assert_eq!(Solution::min_sum_of_lengths(vec![1, 2, 3], 3), 3);
        assert_eq!(
            Solution::min_sum_of_lengths(vec![3, 1, 1, 1, 5, 1, 2, 1], 3),
            3
        );
        assert_eq!(Solution::min_sum_of_lengths(vec![5, 5, 4, 4, 5], 3), -1);
        assert_eq!(
            Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6),
            -1
        );
        assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
        assert_eq!(
            Solution::min_sum_of_lengths(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 2),
            4
        );
    }
}
