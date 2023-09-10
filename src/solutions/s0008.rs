/**
 * # 8. String to Integer (atoi) - https://leetcode.com/problems/string-to-integer-atoi/
 * Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer
 * (similar to C/C++'s atoi function).
 * The algorithm for myAtoi(string s) is as follows:
 * - Read in and ignore any leading whitespace.
 * - Check if the next character (if not already at the end of the string) is '-' or '+'. Read
 * this character in if it is either. This determines if the final result is negative or
 * positive respectively. Assume the result is positive if neither is present.
 * Read in next the characters until the next non-digit character or the end of the input is
 * reached. The rest of the string is ignored.
 * - Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were
 * read, then the integer is 0. Change the sign as necessary (from step 2).
 * - If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the
 * integer so that it remains in the range. Specifically, integers less than -231 should be
 * clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
 * - Return the integer as the final result.
 * ## Note:
 * - Only the space character ' ' is considered a whitespace character.
 * - Do not ignore any characters other than the leading whitespace or the rest of the string
 * after the digits.
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // Trim any leading/trailing whitespaces from the input string.
        let s = s.trim();
        // If the resulting string is empty, return 0.
        if s.is_empty() {
            return 0;
        }
        // Check if the first character of the resulting string is '-' or '+',
        // and set the corresponding flag accordingly.
        let (is_negative, ps) = match s.as_bytes()[0] as char {
            '-' => (true, &s[1..]),
            '+' => (false, &s[1..]),
            _ => (false, s),
        };
        // Find the index of the first non-numeric character in the remaining string.
        let non_numeric_index = ps.find(|c: char| !c.is_numeric()).unwrap_or(ps.len());
        // If there are no numeric characters, return 0.
        if non_numeric_index == 0 {
            return 0;
        }
        // Parse the numeric substring as an integer, and return the result.
        if is_negative {
            s[..non_numeric_index + 1].parse().unwrap_or(i32::MIN)
        } else {
            s[..non_numeric_index].parse().unwrap_or(i32::MAX)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::my_atoi("42".to_owned()), 42)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::my_atoi("       -42".to_owned()), -42)
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::my_atoi("4193 with words".to_owned()), 4193)
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::my_atoi("-91283472332".to_owned()), -2147483648)
    }

    #[test]
    fn example_5() {
        assert_eq!(Solution::my_atoi("words and 987".to_owned()), 0)
    }

    #[test]
    fn example_6() {
        assert_eq!(Solution::my_atoi("3.14159".to_owned()), 3)
    }

    #[test]
    fn example_7() {
        assert_eq!(Solution::my_atoi("+-12".to_owned()), 0)
    }

    #[test]
    fn example_8() {
        assert_eq!(Solution::my_atoi("00000-42a1234".to_owned()), 0)
    }
}
