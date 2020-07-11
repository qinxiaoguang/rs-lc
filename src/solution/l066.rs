pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 数组中的每个数是0-9,表示个十百千万等位的数，对该整体的数进行加一操作
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut over_flow = false;
        let mut res = vec![];
        for i in (0..digits.len()).rev() {
            let mut sum = digits[i]
                + if over_flow { 1 } else { 0 }
                + if i == digits.len() - 1 { 1 } else { 0 };
            sum = if sum >= 10 {
                over_flow = true;
                sum - 10
            } else {
                over_flow = false;
                sum
            };
            res.push(sum);
        }
        if over_flow {
            res.push(1);
        }
        res.reverse();
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l066() {
        println!("{:?}", Solution::plus_one(vec![9, 9, 9]));
    }
}
