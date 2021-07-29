use super::Solution;

impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        // 关键是怎么判断n位数
        // n个0, 但是10位数怎么搞呢？
        let (mut max, mut n) = (1, n);
        while n != 0 {
            max *= 10;
            n -= 1;
        }

        (1..).take(max - 1).collect()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j017() {
        println!(
            "Solution::print_numbers(2) = {:?}",
            Solution::print_numbers(2)
        );
    }
}
