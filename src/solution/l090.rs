pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 求有重复元素的数组的所有子集，包含空集
    // 与78题类似，只不过此题包含了去重的步骤。
    // 如果不想要去重，那只能通过回溯的算法，加上剪枝的思路了。
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut res = HashSet::new();
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
            tmp.sort();
            res.insert(tmp);
        }
        let mut vec_res = vec![];
        res.into_iter().for_each(|v| vec_res.push(v));
        vec_res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l090() {
        println!("{:?}", Solution::subsets(vec![1, 2, 2]));
        println!("{:?}", Solution::subsets(vec![]));
    }
}
