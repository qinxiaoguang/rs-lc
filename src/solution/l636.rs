pub struct Solution {}

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        // 日志其实就是一个栈结构，因为调用函数也是要遵循栈结构，所以使用栈来保存数据，并在每次入栈和出栈都计算时间
        // 但是这个题有点奇葩，当end的时候，其时刻应该加上一，否则计算不准确

        // stack用来保存id，模拟id的出入栈
        let mut stack: Vec<i32> = Vec::with_capacity(logs.len() / 2 + 1);
        let mut res = vec![0; n as usize];
        let mut last_time = 0;

        for log in logs {
            let ps: Vec<&str> = log.split(":").collect();
            let (id, flag, time) = (ps[0], ps[1], ps[2]);
            let (id, mut time) = (id.parse::<i32>().unwrap(), time.parse::<i32>().unwrap());
            if stack.len() != 0 {
                // 获取栈顶元素
                let top = stack[stack.len() - 1];
                // 计算当前距离栈顶元素的时间
                if flag == "end" {
                    time += 1;
                }
                res[top as usize] += time - last_time;
                last_time = time;
            }
            match flag {
                "start" => stack.push(id),
                "end" => {
                    stack.pop();
                }
                _ => {}
            };
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l636() {
        assert_eq!(
            vec![3, 4],
            Solution::exclusive_time(
                2,
                vec![
                    String::from("0:start:0"),
                    String::from("1:start:2"),
                    String::from("1:end:5"),
                    String::from("0:end:6"),
                ],
            )
        );

        /*
        ["0:start:0","0:start:2","0:end:5","0:start:6","0:end:6","0:end:7"]
         */
        assert_eq!(
            vec![8],
            Solution::exclusive_time(
                1,
                vec![
                    String::from("0:start:0"),
                    String::from("0:start:2"),
                    String::from("0:end:5"),
                    String::from("0:start:6"),
                    String::from("0:end:6"),
                    String::from("0:end:7"),
                ],
            )
        );
    }
}
