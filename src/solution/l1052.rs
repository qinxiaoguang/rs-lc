use super::Solution;

impl Solution {
    // 即求能让多数人不生气的一个区间,所有生气的值都有条件变成不生气，而所有不生气的值，却没任何影响
    // 所以customers和grumpy结合，生成一个新数组，该数组中，grumpy[i] == 0时，customers[i] = 0
    // 最终对新的customers数组进行计算，找到minuters区间的最大值
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut res = 0;
        let customers: Vec<i32> = customers
            .into_iter()
            .zip(grumpy.into_iter())
            .into_iter()
            .map(|(c, i)| {
                if i == 1 {
                    c
                } else {
                    res += c;
                    0
                }
            })
            .collect();

        if customers.len() <= minutes as usize {
            return customers.iter().sum::<i32>() + res;
        }
        let mut max: i32 = customers.iter().take(minutes as usize).sum();
        let mut window = max;
        for i in minutes as usize..customers.len() {
            window += customers[i];
            window -= customers[i - minutes as usize];
            max = max.max(window);
        }
        res + max
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1052() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );

        assert_eq!(Solution::max_satisfied(vec![1], vec![0], 1), 1);
    }
}
