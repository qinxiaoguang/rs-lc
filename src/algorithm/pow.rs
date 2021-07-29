/*
快速幂，根据a^n = a^(2^n/2)次方来进行快速运算
*/

fn pow(num: i64, n: i64) -> i64 {
    let (mut res, mut n, mut num) = (1, n, num);
    while n > 0 {
        if n & 1 == 1 {
            res *= num;
        }
        num *= num;
        n >>= 1;
    }
    res
}

fn pow_mod(num: i64, n: i64, MOD: i64) -> i64 {
    let (mut res, mut n, mut num) = (1, n, num);
    while n > 0 {
        if n & 1 == 1 {
            res = res * num % MOD;
        }
        num = num * num % MOD;
        n >>= 1;
    }
    res
}
