use super::Solution;

impl Solution {
    // 如果只有一个数出现1次，那么可以使用异或来进行运算，但是此题是出现两个一次的数
    // 那么对所有结果异或后，将得到 a ^ b(rust中^符号表示异或)
    // 将数组分为两组，且保证两组的数中只有一个出现1次的数，关键就是怎么分组
    // 将a^b写成二进制的方式，取低位第一个为1的索引，假如索引是1，以此为条件进行分组
    // 索引处都是1的都分到第一组，索引处都是0的分到第二组，那么这样的话，相同的数肯定会分到相同的组
    // 同时，a和b也会被分到不同组中。
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let ab = nums.iter().fold(0, |a, b| a ^ *b);
        let mut index = 0;
        loop {
            if ab >> index & 1 == 1 {
                break;
            }
            index += 1;
        }
        let (mut left, mut right) = (0, 0);
        for num in nums {
            if num >> index & 1 == 1 {
                left ^= num;
            } else {
                right ^= num;
            }
        }
        vec![left, right]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l260() {
        assert_eq!(Solution::single_number(vec![3, 5]), vec![3, 5]);
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
        assert_eq!(
            Solution::single_number(vec![-1, -1, -2, -2, 0, 3]),
            vec![3, 0]
        );
    }
}
