// 最大公约数: gcd(a,b) = gcd(b,a%b),
// a = b*q + p , 可知gcd(b,p)即能整除b，又能整除a
// 而q = a-b*p, 所以gcd(a,b)又整除gcd(b,p)，所以gcd(a,b) = gcd(b,p)
// p = a%b, => gcd(a,b) = gcd(b, a%b)

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
