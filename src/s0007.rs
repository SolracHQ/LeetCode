/**
 * # 7. Reverse Integer - https://leetcode.com/problems/reverse-integer/
 * Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x.abs(); // make a copy of the absolute value of the input integer
        let mut rev = 0; // initialize a variable to store the reversed integer

        while num != 0 { // loop until all digits have been processed
            let digit = num % 10; // get the rightmost digit of the input integer
            num /= 10; // remove the rightmost digit from the input integer
            // Check for overflow
            if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && digit > 7) {
                // if the reversed integer is greater than the maximum 32-bit signed integer,
                // or if the reversed integer is equal to the maximum 32-bit signed integer
                // and the rightmost digit of the input integer is greater than 7,
                // return 0 to indicate overflow
                return 0;
            }
            if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && digit < -8) {
                // if the reversed integer is less than the minimum 32-bit signed integer,
                // or if the reversed integer is equal to the minimum 32-bit signed integer
                // and the rightmost digit of the input integer is less than -8,
                // return 0 to indicate overflow
                return 0;
            }
            rev = rev * 10 + digit; // add the rightmost digit to the reversed integer
        }
        if x.is_negative() { // if the input integer was negative
            -rev // return the negation of the reversed integer
        } else {
            rev // otherwise, return the reversed integer
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::reverse(123),
            321
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::reverse(-123),
            -321
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::reverse(120),
            21
        )
    }
}