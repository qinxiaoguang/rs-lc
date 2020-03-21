pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 求无重复元素的数组的所有子集，包含空集
    // 使用二进制
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for mut i in 0..(2_i32.pow(nums.len() as u32)) {
            let mut index = 0;
            let mut tmp = vec![];
            while i != 0 {
                if i & 1 == 1 {
                    tmp.push(nums[index]);
                }
                i >>= 1;
                index += 1;
            }
            res.push(tmp);
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l078() {
        println!("{:?}", Solution::subsets(vec![1, 2, 3]));
        println!("{:?}", Solution::subsets(vec![]));
    }
}
