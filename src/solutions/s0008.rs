/**
* 8. String to Integer (atoi)
*
* Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer.

*
* Example 1:
* Input: s = "42"
* Output: 42
*
* Example 2:
* Input: s = " -042"
* Output: -42
*
* Example 3:
* Input: s = "1337c0d3"
* Output: 1337
*
* Example 4:
* Input: s = "0-1"
* Output: 0
*
* Example 5:
* Input: s = "words and 987"
* Output: 0

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
        // Input: s = "42"
        // Expected: 42
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn example_2() {
        // Input: s = " -042"
        // Expected: -42
        assert_eq!(Solution::my_atoi(" -042".to_string()), -42);
    }

    #[test]
    fn example_3() {
        // Input: s = "1337c0d3"
        // Expected: 1337
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
    }

    #[test]
    fn example_4() {
        // Input: s = "0-1"
        // Expected: 0
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
    }

    #[test]
    fn example_5() {
        // Input: s = "words and 987"
        // Expected: 0
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
}
