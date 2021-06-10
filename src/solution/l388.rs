use super::Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut lines: Vec<&str> = input.split('\n').collect();
        let mut d = [0i32; 104];
        let mut res = 0i32;
        for mut line in lines {
            let mut t_cnt = 1;
            while line.starts_with("\t") {
                line = &line[1..];
                t_cnt += 1;
            }
            // t_cnt就是层数,若t_cnt是0， 则是最顶层
            if line.contains(".") {
                // 文件
                res = res.max(line.len() as i32 + d[t_cnt - 1] + if t_cnt == 1 { 0 } else { 1 });
                continue;
            }
            d[t_cnt] = line.len() as i32 + d[t_cnt - 1] + if t_cnt == 1 { 0 } else { 1 };
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l388() {
        assert_eq!(
            Solution::length_longest_path(String::from("file1.txt\nfile2.txt")),
            9
        );

        assert_eq!(Solution::length_longest_path(String::from("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext")),32);
        assert_eq!(Solution::length_longest_path(String::from("a")), 0);
        assert_eq!(Solution::length_longest_path(String::from("rzzmf\nv\n\tix\n\t\tiklav\n\t\t\ttqse\n\t\t\t\ttppzf\n\t\t\t\t\tzav\n\t\t\t\t\t\tkktei\n\t\t\t\t\t\t\thhmav\n\t\t\t\t\t\t\t\tbzvwf.txt")), 47);
    }
}
