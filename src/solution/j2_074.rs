use super::*;

impl Solution {
    // 给出一个区间的集合，请合并所有重叠的区间。
    // 其实就是依次合并两个区间，但是先要对整个数组进行排序，排序规则是按左值排序，慢慢向下遍历
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if intervals.is_empty() {
            return res;
        }
        let mut intervals = intervals;
        intervals.sort();
        let mut last = intervals[0].clone();
        for i in 1..intervals.len() {
            let (left1, left2) = (last[0], last[1]);
            let (right1, right2) = (intervals[i][0], intervals[i][1]);
            if left2 >= right1 {
                let (x, y) = Self::merge_two((left1, left2), (right1, right2));
                last = vec![x, y];
            } else {
                res.push(last);
                last = intervals[i].clone()
            }
        }
        res.push(last);
        res
    }

    // merge两个区间前提是left2 <= right1
    fn merge_two((left1, left2): (i32, i32), (right1, right2): (i32, i32)) -> (i32, i32) {
        let min = std::cmp::min(left1, right1);
        let max = std::cmp::max(left2, right2);
        (min, max)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_074() {
        println!(
            "{:?}",
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18],])
        );
        println!("{:?}", Solution::merge(vec![vec![1, 4], vec![4, 5],]));
        println!("{:?}", Solution::merge(vec![vec![1, 4]]));
        println!("{:?}", Solution::merge(vec![]));
        println!(
            "{:?}",
            Solution::merge(vec![vec![1, 2], vec![2, 3], vec![3, 4]])
        );
        println!("{:?}", Solution::merge(vec![vec!(1, 4), vec![0, 0]]));
        println!("{:?}", Solution::merge(vec![vec!(1, 4), vec![0, 5]]));
    }
}
