use super::Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut point = (0, 0);
        for i in moves.chars() {
            match i {
                'U' => point.0 += 1,
                'D' => point.0 -= 1,
                'L' => point.1 += 1,
                'R' => point.1 -= 1,
                _ => {}
            }
        }
        point == (0, 0)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l657() {}
}
