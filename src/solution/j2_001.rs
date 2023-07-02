use super::Solution;

impl Solution {
    // 计算a/b的结果，不允许使用乘除等方法
    // 关键点太多了， 首先对于除法结果溢出， 只有两个数都在边界的时候才会存在
    // 所以对于a为MIN或MAX及b为1，-1的情况，直接写死。
    // 第二个边界是b+b的边界，可能也会溢出
    // 第三个边界是当a为负数最大值，转为正数的时候也会溢出，所以干脆以负数的方式来进行计算。
    pub fn divide(a: i32, b: i32) -> i32 {
        if a == i32::MIN {
            if b == 1 {
                return i32::MIN;
            }
            if b == -1 {
                return i32::MAX;
            }
        }
        if a == i32::MAX {
            if b == -1 {
                return -i32::MAX;
            }
            if b == 1 {
                return i32::MAX;
            }
        }

        let mut neg = 0;
        let (mut a,mut b) = (a,b);
        if a>0 {
            a = -a;
            neg -= 1;
        }
        if b>0 {
            b = -b;
            neg -= 1;
        }
        neg = -neg;
        if a > b {
            return 0;
        }
        let (mut res,_) = Self::neg_divide(a, b);
        (0..neg).for_each(|_|{res = -res;});
        res
    }

    // 因为整数中负数比正数绝对值要大， 所以使用负数来进行计算
    fn neg_divide(a:i32,b:i32) -> (i32,i32) {
       if a > b {
            return (0,a);
        }
        // b+b有可能溢出
        if a <= b && (i32::MIN-b>b||a> b+b) {
            return (1,a-b);
        }
        if i32::MIN -b > b { 
            return (0,a);
        }
        let (mut res,last) = Self::neg_divide(a, b+b);
        res += res;
        if last <= b {
            return (res+1,last-b);
        }
        return (res,last);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_001() {
        assert_eq!(Solution::divide(15, 2),7);
        assert_eq!(Solution::divide(7, -3),-2);
        assert_eq!(Solution::divide(0, 1),0);
        assert_eq!(Solution::divide(1, 1),1);
        assert_eq!(Solution::divide(-1, 1),-1);
        assert_eq!(Solution::divide(2147483647, 1),2147483647);
        assert_eq!(Solution::divide(2147483647, 2),1073741823);
        assert_eq!(Solution::divide(-2147483648, 2),-1073741824);
        assert_eq!(Solution::divide(-231, 3),-77);
    }
}
