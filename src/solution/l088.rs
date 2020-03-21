pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 合并两个有序数组，其中m是num1中有效数的个数，n是num2有效数的个数。
    // 可以假设num1的长度一定能容纳num2的长度
    // 其实还是使用双指针。只不过绕点路。
    // 不用额外空间的话，需要从后往前，找最大的值，放在nums1的最后,如果从前往后，会影响到nums1的遍历。
    // 这个方法真是太妙了，思考一下，你为什么想不到呢。
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut p1, mut p2) = (m - 1, n - 1);
        if n == 0 {
            return;
        }
        let mut tail = m + n - 1;
        loop {
            if p1 < 0 {
                while p2 >= 0 && tail >= 0 {
                    nums1[tail as usize] = nums2[p2 as usize];
                    p2 -= 1;
                    tail -= 1;
                }
                break;
            }
            if p2 < 0 {
                while p1 >= 0 && tail >= 0 {
                    nums1[tail as usize] = nums1[p1 as usize];
                    p1 -= 1;
                    tail -= 1;
                }
                break;
            }

            if nums1[p1 as usize] >= nums2[p2 as usize] {
                nums1[tail as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[tail as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
            tail -= 1;
            println!("nums1:{:?},nums2:{:?}, p1:{},p2:{}", nums1, nums2, p1, p2);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l088() {
        let mut nums1 = vec![1, 2, 4, 5, 6, 0];
        let mut nums2 = vec![3];
        Solution::merge(&mut nums1, 5, &mut nums2, 1);
        println!("{:?}", nums1);

        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let mut nums2 = vec![1, 2, 3];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        println!("{:?}", nums1);

        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        println!("{:?}", nums1);

        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        println!("{:?}", nums1);
    }
}
