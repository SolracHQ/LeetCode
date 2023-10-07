/**
 * Problem 50. Pow(x, n)
 * In this problem, we need to implement a function to calculate the power of a number.
*/
struct Solution;

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

mod test {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::my_pow(2.10000, 3), 2.10000_f64.powi(3));
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::my_pow(2.00000, -2), 0.25000);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::my_pow(2.00000, -2147483648), 0.0);
    }
}
