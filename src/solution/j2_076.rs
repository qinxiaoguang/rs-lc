use super::*;

impl Solution {
    // 查找数组中第k大的数，正常思路就是使用小顶堆
    // 而更优思路却是使用快排的思想
    // 快排每次都会把数组拆分成比他小和比他大的两个数组，比他小的数的个数如果刚好等于k-1那么当前选择的值就是结果
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        return Self::_q_fake_sort(&mut nums, k).unwrap();
    }

    fn _q_fake_sort(mut nums: &mut [i32], k: i32) -> Option<i32> {
        if nums.len() == 1 && k == 1 {
            return Some(nums[0]);
        }

        if nums.len() == 2 {
            if nums[0] > nums[1] {
                nums.swap(0, 1);
            }

            if k <= 2 {
                return Some(nums[2 - k as usize]);
            }
            return None;
        }
        // 分隔值是nums[0]
        let (mut l, mut r) = (0, nums.len() - 1);

        // 比他小的放左边， 比他大的放右边
        while l < r {
            // 从右侧找到比他小的,当前基点是nums[l]
            while l < r && nums[l] < nums[r] {
                r -= 1;
            }
            if l < r {
                // swap后基点在nums[r]
                nums.swap(l, r);
                l += 1;
            }

            // 当前基点是nums[r]
            while l < r && nums[l] < nums[r] {
                l += 1;
            }
            if l < r {
                nums.swap(l, r);
                r -= 1;
            }
        }
        // 最终目标值落到了l处
        if k as usize == nums.len() - l {
            return Some(nums[l]);
        }

        // 结果在右边
        if nums.len() - l > k as usize {
            return Self::_q_fake_sort(&mut nums[l + 1..], k as i32);
        }
        // 结果在左边
        let lastk = k - (nums.len() - l) as i32;
        return Self::_q_fake_sort(&mut nums[0..l as usize], lastk);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_076() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(Solution::find_kth_largest(vec![2, 1], 1), 2);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
