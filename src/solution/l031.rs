pub struct Solution {}
impl Solution {
    // 经典问题，下一个排列
    // 方法就是，从后往前找到第一个不是升序的数，第一个不是升序的，即第一个比较小的数和后边的比该数刚好大一点的数交换，
    // 因为如果跟比自己小的数交换，结果还是比自己小,而如果跟比自己大很多的交换，结果就不是下一个排列。
    // 所以需要用比自己小点的数进行交换，而为什么需第一个不是升序的呢，因为假如选择升序中的某个数时，不管与后边的哪个数交换，他都比当前的数要小，所以不符合结果
    // 而在交换后，将该位置后边的所有的数重新按照从小到大的顺序排列，其实就是对其后所有的数reverse即可。
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let len = nums.len();
        let (mut i, mut j, mut k) = ((len - 2) as i32, (len - 1) as i32, (len - 1) as i32);
        while i >= 0 && nums[i as usize] >= nums[j as usize] {
            i -= 1;
            j -= 1;
        }
        if i >= 0 {
            while nums[i as usize] >= nums[k as usize] {
                k -= 1;
            }
            nums.swap(i as usize, k as usize);
        }
        (&mut nums[j as usize..len]).reverse();
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l031() {
        let mut nums = vec![1, 2, 3, 4, 5];
        Solution::next_permutation(&mut nums);
        let mut nums = vec![1, 1];
        Solution::next_permutation(&mut nums);
        println!("{:?}", nums);
    }
}
