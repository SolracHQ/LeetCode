/*
 * 29. Divide Two Integers - https://leetcode.com/problems/divide-two-integers/
 * 
 * Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.
 *
 * The integer division should truncate toward zero, which means losing its fractional part. For example, 8.345 would be truncated to 8, 
 * and -2.7335 would be truncated to -2.
 *
 * Return the quotient after dividing dividend by divisor.
 *
 * Note: Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [−231, 231 − 1]. 
 * For this problem, if the quotient is strictly greater than 231 - 1, then return 231 - 1, and if the quotient is strictly less than -231, 
 * then return -231.
*/
struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let (mut a, b) = match (dividend, divisor) {
            (i32::MIN, -1) => {return i32::MAX;}
            (i32::MIN, i32::MIN) => {return 1;}
            (i32::MIN, i32::MAX) => {return -1;}
            (_, 1) => {return dividend;}
            (_, -1) => {return -dividend;}
            (i32::MIN, _) => (i32::MAX.abs() as u32 + 1, divisor.abs() as u32),
            _ => (dividend.abs() as u32, divisor.abs() as u32)
        };
        let mut result = 0;
        let is_negative = dividend.is_negative() ^ divisor.is_negative();
        while a >= b {
            let mut tmp = 0;
            while a >= b<<(tmp+1) && (tmp+1) < b.leading_zeros() {
                tmp += 1;
            }
            result += 1 << tmp;
            a -= b << tmp;
        }
        if !is_negative {
            result
        } else {
            -result
        }
    }
}

mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::divide(10, 3),
            3
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::divide(7, -3),
            -2
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::divide(-2147483648, -1),
            2147483647
        )
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::divide(-2147483648, 2),
            -1073741824
        )
    }
}