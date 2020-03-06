pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 针对一个有序的区间，插入一个区间，并输出无重叠区间
    // 其实就是056题，只不过是已经排过序了
    // 所以其实只需要遍历一遍
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.push(new_interval);
        Self::merge(intervals)
    }

    // 给出一个区间的集合，请合并所有重叠的区间。
    // 其实就是依次合并两个区间，但是先要对整个数组进行排序，排序规则是按左值排序，慢慢向下遍历
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if intervals.is_empty() {
            return res;
        }
        let mut intervals = intervals;
        intervals.sort();
        println!("sort:{:?}", intervals);
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
    fn test_l057() {
        println!(
            "{:?}",
            Solution::insert(vec![vec![1, 3], vec![6, 9],], vec![2, 5])
        );
    }
}
