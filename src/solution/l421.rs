use super::Solution;

#[derive(Debug)]
struct Tire {
    root: Node,
}

#[derive(Debug)]
struct Node {
    data: Option<i32>,
    children: [Option<Box<Node>>; 2],
}

impl Tire {
    fn new() -> Self {
        Tire {
            root: Node {
                data: None,
                children: [None, None],
            },
        }
    }

    fn add(&mut self, data: i32) {
        let mut i = 31;
        let mut curr = &mut self.root;
        while i >= 0 {
            let next = if data & (1 << i) == 0 {
                // 该位是0
                0usize
            } else {
                // 该位是1
                1usize
            };
            if curr.children[next].is_none() {
                curr.children[next] = Some(Box::new(Node {
                    data: None,
                    children: [None, None],
                }))
            }
            curr = &mut *curr.children[next].as_mut().unwrap();
            i -= 1;
        }
        curr.data = Some(data);
    }

    fn find(&mut self, target: i32) -> i32 {
        // 找一个跟target的路径基本相反的值
        let mut i = 31;
        let mut curr = &mut self.root;
        while i >= 0 {
            let mut next = if target & (1 << i) == 0 {
                // 该位是0,则找1
                1usize
            } else {
                // 该位是1,则找0
                0usize
            };
            if curr.children[next].is_none() {
                // 如果目标找不到，则找另一条路
                next = 1 - next;
            }
            curr = &mut *curr.children[next].as_mut().unwrap();
            i -= 1;
        }
        // 一定能找到一个值
        curr.data.unwrap()
    }
}

impl Solution {
    // 构造Trie树,每次从树中找到与该数相反反向的数
    // 最次的树的深度是32，但可以优化,优化就算了,就这么滴吧
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut t = Tire::new();
        let mut res = -1;
        for num in nums {
            t.add(num);
            res = res.max(num ^ t.find(num));
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l421() {
        assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
        assert_eq!(Solution::find_maximum_xor(vec![0]), 0);
        assert_eq!(Solution::find_maximum_xor(vec![2, 4]), 6);
        assert_eq!(Solution::find_maximum_xor(vec![8, 10, 2]), 10);
        assert_eq!(Solution::find_maximum_xor(vec![1]), 0);
        assert_eq!(
            Solution::find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]),
            127
        );
    }
}
