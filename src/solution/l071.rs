pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 简化路径，.. 则为上一层，.为此层
    // 利用栈
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        for s in path.split("/") {
            match s {
                "." | "" => {
                    continue;
                }
                ".." => {
                    stack.pop();
                }
                _ => {
                    stack.push(s);
                }
            }
        }
        "/".to_string() + &stack.join("/")
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l071() {
        assert_eq!("/home", Solution::simplify_path("/home/".to_string()));
        assert_eq!("/", Solution::simplify_path("/../".to_string()));
        assert_eq!("/c", Solution::simplify_path("/a/./b/../../c/".to_string()));
        assert_eq!(
            "/a/b/c",
            Solution::simplify_path("/a//b////c/d//././/..".to_string())
        );
        assert_eq!(
            "/home/foo",
            Solution::simplify_path("/home//foo".to_string())
        );
    }
}
