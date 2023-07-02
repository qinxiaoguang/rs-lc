use super::*;

impl Solution {
    // 获取一个数组的所有子集
    // 利用递归的方式，先固定第一个数，求第一数后边所有数的子集
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::_subsets(&nums)
    }

    fn _subsets(nums: &[i32]) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]];
        for i in (0..nums.len()).rev() {
            let sub_ans = Self::_subsets(&nums[..i]);
            sub_ans.into_iter().for_each(|mut v| {
                v.push(nums[i]);
                ans.push(v)
            });
        }
        return ans;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_079() {
        println!("{:?}", Solution::subsets(vec![1, 2, 3]));
        println!("{:?}", Solution::subsets(vec![0]));
    }
}
