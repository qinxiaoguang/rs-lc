pub struct Solution {}
impl Solution {
    pub fn test() {
        let mut v = vec!["haha".to_string()];
        Self::self_build(0, &mut v);
    }

    fn self_build(mut cnt: i32, param: &mut Vec<String>) {
        if cnt == 5 {
            return;
        }
        cnt += 1;
        param.push("haha".to_string());
        Self::self_build(cnt, param);
        println!("{:?}", param);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l000() {}
}
