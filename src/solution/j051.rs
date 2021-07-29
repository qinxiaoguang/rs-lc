use super::Solution;

impl Solution {
    // 数组的逆序对
    // 考虑冒泡排序，每次选出最小值，放到最前边
    // 其冒泡过程就是在找当前最小值与前边值组成的逆序对数
    // 所以逆序对，就是求排序过程交换次数
    // 但需要的是稳定排序
    // 使用冒泡，则需要O(n^2)的复杂度
    // 所以可以考虑使用归并(快排不是稳定，考虑[5,4,4]的情况，快排可能会把最后一个4放到最前)
    // 归并的话，需要在归并的时候 计算左右两侧相对的逆序对数
    // 每次右侧在往数组添加的时候，计算该数值对应的逆序对数即可
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        Self::j051_merge(&nums, &mut res);
        res
    }

    // merge
    fn j051_merge(nums: &[i32], res: &mut i32) -> Vec<i32> {
        if nums.len() <= 1 {
            return Vec::from(nums);
        }
        if nums.len() == 2 {
            if nums[0] > nums[1] {
                *res += 1;
                return vec![nums[1], nums[0]];
            }
            return Vec::from(nums);
        }

        // 分离
        let mid = (0 + nums.len()) / 2;
        let lnum = Self::j051_merge(&nums[..mid], res);
        let rnum = Self::j051_merge(&nums[mid..], res);
        // 归并
        let mut resnum = Vec::new();
        let (mut l, mut r) = (0, 0);
        while l < lnum.len() && r < rnum.len() {
            if lnum[l] <= rnum[r] {
                resnum.push(lnum[l]);
                l += 1;
            } else {
                // r小
                *res += (lnum.len() - l) as i32;
                resnum.push(rnum[r]);
                r += 1;
            }
        }

        lnum[l..].iter().for_each(|n| resnum.push(*n));
        rnum[r..].iter().for_each(|n| resnum.push(*n));
        resnum
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j051() {
        assert_eq!(Solution::reverse_pairs(vec![7, 5, 6, 4]), 5);
        assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 4);
        assert_eq!(Solution::reverse_pairs(vec![]), 0);
    }
}
