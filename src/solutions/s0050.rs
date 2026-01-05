/**
* 50. Pow(x, n)
*
* Implement pow(x, n), which calculates x raised to the power n (i.e., x^n).

*
* Example 1:
* Input: x = 2.00000, n = 10
* Output: 1024.00000
*
* Example 2:
* Input: x = 2.10000, n = 3
* Output: 9.26100
*
* Example 3:
* Input: x = 2.00000, n = -2
* Output: 0.25000

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // Handle edge cases
        if x == 0.0 || x == 1.0 {
            return x;
        }
        if x == -1.0 {
            return if n & 1 == 0 { -x } else { x };
        }

        let (mut base, mut exp) = match n {
            // Anything to the power of 0 is 1
            0 => return 1.0,
            // Negative exponents are the same as 1 over the positive exponent
            n if n < 0 => (1.0 / x, -(n as i64)),
            _ => (x, n as i64),
        };
        let mut result = 1.0;

        // Fast power algorithm
        while exp > 0 {
            // If the current bit is 1, multiply the result by the current base
            if exp & 1 == 1 {
                result *= base;
            }
            // Square the base and divide the exponent by 2
            base *= base;
            exp >>= 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: x = 2.00000, n = 10
        // Expected: 1024.00000
        let ans = Solution::my_pow(2.0, 10);
        assert!((ans - 1024.0).abs() < 1e-9);
    }

    #[test]
    fn example_2() {
        // Input: x = 2.10000, n = 3
        // Expected: 9.26100
        let ans = Solution::my_pow(2.1, 3);
        assert!((ans - 9.261).abs() < 1e-9);
    }

    #[test]
    fn example_3() {
        // Input: x = 2.00000, n = -2
        // Expected: 0.25000
        let ans = Solution::my_pow(2.0, -2);
        assert!((ans - 0.25).abs() < 1e-9);
    }
}
