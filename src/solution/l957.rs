use super::Solution;

impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        // 如果要优化，优化方式就是采用位
        // 另外，题目中的N特别大，所以暴力是解决不了的。
        // 解决N特别的题，一般会找规律，此题的规律是，循环到某个时间的时候，数组就开始重复
        // 所以只需要找到数组变成原样时，经过了几天.并保存循环的数组
        let mut last = cells;
        let mut cells = last.clone();
        let mut map = std::collections::HashMap::new();
        let mut days = vec![];
        map.insert(cells.clone(), 0);
        days.push(cells.clone());
        let l = cells.len();
        for i in 1..=n {
            cells[0] = 0;
            if l - 1 >= 0 {
                cells[l - 1] = 0;
            }
            for i in 1..l - 1 {
                cells[i] = if last[i - 1] == last[i + 1] { 1 } else { 0 }
            }
            if map.contains_key(&cells) {
                let repeat_from: i32 = *map.get(&cells).unwrap();
                let repeat_num = i - repeat_from;
                return days[((n - i) % repeat_num) as usize + repeat_from as usize].clone();
            }
            days.push(cells.clone());
            map.insert(cells.clone(), i);
            std::mem::swap(&mut last, &mut cells);
        }
        last
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l957() {
        assert_eq!(
            Solution::prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
            vec![0, 0, 1, 1, 0, 0, 0, 0]
        );

        assert_eq!(
            Solution::prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
            vec![0, 0, 1, 1, 1, 1, 1, 0]
        );
    }
}
