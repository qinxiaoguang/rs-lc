pub struct Solution {}
impl Solution {
    // 寻找两个数组的中位数: 如[1,3] ,[2]的中位数是 2，即[1,2,3] => 2
    // 就是给两个数组，长度总和为k,无论k是奇还是偶，都是找(k+1)/2,(k+2)/2个数。所以先找第(k-1)/2 + 1个数，
    // 方法就是先找两个数组中的中位数，如a,b，如果a<b,那么a的数组中小于a的数字一定没有第(k-1)/2 + 1个数。所以依次递归即可。
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let length = nums1.len() + nums2.len();
        let (k1, k2) = ((length + 1) / 2, (length + 2) / 2);
        if nums1.len() == 0 {
            return (nums2[k1 - 1] as f64 + nums2[k2 - 1] as f64) / 2 as f64;
        } else if nums2.len() == 0 {
            return (nums1[k1 - 1] as f64 + nums1[k2 - 1] as f64) / 2 as f64;
        }
        let (res1, res2) = (
            Self::find_k(&nums1, &nums2, k1),
            Self::find_k(&nums1, &nums2, k2),
        );
        (res1 + res2) / 2 as f64
    }

    // 找第k个数，注意k不能是0
    fn find_k(nums1: &[i32], nums2: &[i32], k: usize) -> f64 {
        //mid向下取整
        // [1] [2,3,4,5,6] 4
        let mut mid = if k <= 1 { 0 } else { k / 2 - 1 }; // mid =1
        if mid >= nums1.len() {
            mid = nums1.len() - 1;
        }
        if mid >= nums2.len() {
            mid = nums2.len() - 1;
        }
        if nums1[mid] < nums2[mid] {
            if k == 1 {
                return nums1[mid] as f64;
            }
            if mid == nums1.len() - 1 {
                if mid == k - 1 {
                    return nums1[mid] as f64;
                }
                return nums2[k - mid - 2] as f64;
            }
            Self::find_k(&nums1[mid + 1..], &nums2, k - mid - 1)
        } else {
            if k == 1 {
                return nums2[mid] as f64;
            }
            if mid == nums2.len() - 1 {
                if mid == k - 1 {
                    return nums2[mid] as f64;
                }
                return nums1[k - mid - 2] as f64;
            }
            Self::find_k(&nums1, &nums2[mid + 1..], k - mid - 1)
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l004() {
        assert_eq!(
            2.0_f64,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
        );
        assert_eq!(
            2.0_f64,
            Solution::find_median_sorted_arrays(vec![2], vec![1, 3])
        );
        assert_eq!(
            2.5_f64,
            Solution::find_median_sorted_arrays(vec![2], vec![3])
        );
        assert_eq!(
            3.0_f64,
            Solution::find_median_sorted_arrays(vec![], vec![3])
        );

        assert_eq!(
            2.5_f64,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );
        assert_eq!(
            2.5_f64,
            Solution::find_median_sorted_arrays(vec![1, 4], vec![2, 3])
        );
        assert_eq!(
            3.5_f64,
            Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4, 5, 6])
        );
    }
}
