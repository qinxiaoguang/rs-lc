use rand::prelude::*;
struct Solution {
    pre: Vec<i32>,
}

// 按权重生成随机数
// 如数组[1,3]其中下标0出现的概率是1/4,下标1出现的概率是3/4
// 构造前缀和并随机生成一个随机数，在前缀和中使用二分查找找到对应位置
// 如上述例子，前缀和是[1,4],随机数如果是0-1，则返回0，而如果是1-4，则返回1
impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut sum = 0;
        Self {
            pre: w
                .iter()
                .map(|x| {
                    sum += *x;
                    sum
                })
                .collect(),
        }
    }

    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..*self.pre.last().unwrap());
        let (mut l, mut r) = (0, self.pre.len() - 1);

        while l < r {
            let mid = (l + r) / 2;
            if self.pre[mid] > random {
                r = mid;
                continue;
            }
            l = mid + 1;
        }
        return l as i32;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_071() {
        let s = Solution::new(vec![1]);
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
        let s = Solution::new(vec![1, 3]);
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
        println!("{}", s.pick_index());
    }
}
