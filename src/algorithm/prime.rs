/*
prime相关算法
*/

// 最暴力算法，2~sqrt(n)
pub fn is_prime(n: i32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    n != 1
}

// 埃氏筛，O(nloglogn)复杂度，基本上线性
// 核心思想是，每次从数组中选出最小的素数为p, 将数组中所有p的倍数都删掉，重复该步骤
// 计算n以内的素数个数,也可以在其中添加素数表
pub fn aishi_prime(n: i64) -> i32 {
    // MAX 可以自定义
    let mut is_prime = vec![true; usize::MAX];
    let mut res = 0;
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if is_prime[i as usize] {
            // 也可以在这个地方添加素数表，每次res+1时，就成功捕获一个素数
            res += 1;
            for j in (2 * i..=n).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }
    res
}
