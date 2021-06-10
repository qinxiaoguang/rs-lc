use super::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // O(1)的空间复杂度
        let l = s.len();
        for i in 0..l / 2 {
            s.swap(i, l - i - 1);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l344() {
        let mut v = vec!['a', 'b', 'c'];
        Solution::reverse_string(&mut v);
        assert_eq!(v, vec!['c', 'b', 'a']);

        let mut v = vec!['a', 'b'];
        Solution::reverse_string(&mut v);
        assert_eq!(v, vec!['b', 'a']);
    }
}
