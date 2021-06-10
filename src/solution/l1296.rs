use super::Solution;

impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        // 贪心，最小的数必然在某个数组的第一个，所以每次从数组中找最小的数，看是否能组成一个k个连续的数组
        // 可以使用两个队列来搞，从第一个队列中取数值，符合条件弹出，不符合条件，弹到另一队列即可
        if nums.len() % k as usize != 0 {
            return false;
        }
        let mut nums = nums;
        nums.sort();
        let mut q1 = std::collections::VecDeque::new();
        let mut q2 = q1.clone();

        for num in nums {
            q1.push_back(num);
        }

        let mut cnt = 0;
        let mut next_v = -1;
        loop {
            if q1.is_empty() && q2.is_empty() {
                break;
            }
            while let Some(v) = q1.pop_front() {
                if next_v == -1 {
                    next_v = v + 1;
                    cnt += 1;
                    continue;
                }
                if v > next_v {
                    return false;
                } else if v < next_v {
                    q2.push_back(v);
                } else {
                    // v == next_v
                    cnt += 1;
                    cnt %= k;
                    if cnt == 0 {
                        next_v = -1;
                        // 将q2的内容再放入q1
                        q2.append(&mut q1);
                        q1.clear();
                        std::mem::swap(&mut q1, &mut q2);
                    } else {
                        next_v += 1;
                    }
                }
            }
            if cnt != 0 {
                return false;
            }
            q2.append(&mut q1);
            q1.clear();
            std::mem::swap(&mut q1, &mut q2);
        }
        return cnt == 0;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1296() {
        assert_eq!(
            Solution::is_possible_divide(vec![1, 2, 3, 3, 4, 4, 5, 6], 4),
            true
        );

        assert_eq!(
            Solution::is_possible_divide(vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3),
            true
        );

        assert_eq!(
            Solution::is_possible_divide(vec![3, 3, 2, 2, 1, 1], 3),
            true
        );
        assert_eq!(
            Solution::is_possible_divide(vec![3, 3, 2, 2, 1, 5], 3),
            false
        );
    }
}
