/**
* 29. Divide Two Integers
*
* Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.

*
* Example 1:
* Input: dividend = 10, divisor = 3
* Output: 3
*
* Example 2:
* Input: dividend = 7, divisor = -3
* Output: -2

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let (mut a, b) = match (dividend, divisor) {
            (i32::MIN, -1) => {
                return i32::MAX;
            }
            (i32::MIN, i32::MIN) => {
                return 1;
            }
            (i32::MIN, i32::MAX) => {
                return -1;
            }
            (_, 1) => {
                return dividend;
            }
            (_, -1) => {
                return -dividend;
            }
            (i32::MIN, _) => (i32::MAX.abs() as u32 + 1, divisor.abs() as u32),
            _ => (dividend.abs() as u32, divisor.abs() as u32),
        };
        let mut result = 0;
        let is_negative = dividend.is_negative() ^ divisor.is_negative();
        while a >= b {
            let mut tmp = 0;
            while a >= b << (tmp + 1) && (tmp + 1) < b.leading_zeros() {
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

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: dividend = 10, divisor = 3
        // Expected: 3
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn example_2() {
        // Input: dividend = 7, divisor = -3
        // Expected: -2
        assert_eq!(Solution::divide(7, -3), -2);
    }
}
