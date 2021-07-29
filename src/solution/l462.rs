use super::Solution;

impl Solution {
    // 其实就是找到一个数，使得所有数距离该数的和最小
    // 刚开始考虑成了方差题，做了半天没过
    // 后来也是看了题解，才知道有这么一条规律
    // 平面上有很多点，找一条平行线，使得所有点距离该线之和最小
    // 这么一条线应该是所有点中的高度的中位数，如果点的个数是奇数个
    // 则中位数就是最中间的那个数
    // 否则是中间的两个数区间内的任意一个数
    // 所以我们查找中位数即可
    // 有两种算法，第一种排序
    // 第二种借鉴快排的思想，挑选一个基准数a，比a小的放左边，比a大的放右边
    // 并比较两边数的长度，判断中位数在左侧还是右侧,并继续选择
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let base = nums[nums.len() / 2];
        nums.iter().map(|x| (*x - base).abs()).sum()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l462() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
        assert_eq!(Solution::min_moves2(vec![1, 2, 4]), 3);
        assert_eq!(Solution::min_moves2(vec![1, 0, 0, 8, 6]), 14);
    }
}
