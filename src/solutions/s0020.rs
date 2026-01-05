/**
* 20. Valid Parentheses
*
* Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

*
* Example 1:
* Input: s = "()"
* Output: true
*
* Example 2:
* Input: s = "()[]{}"
* Output: true
*
* Example 3:
* Input: s = "(]"
* Output: false
*
* Example 4:
* Input: s = "([])"
* Output: true
*
* Example 5:
* Input: s = "([)]"
* Output: false

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for char in s.as_bytes() {
            match char {
                b'{' | b'(' | b'[' => stack.push(*char),
                b')' if stack.pop() == Some(b'(') => (),
                b']' if stack.pop() == Some(b'[') => (),
                b'}' if stack.pop() == Some(b'{') => (),
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: s = "()"
        // Expected: true
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn example_2() {
        // Input: s = "()[]{}"
        // Expected: true
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn example_3() {
        // Input: s = "(]"
        // Expected: false
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn example_4() {
        // Input: s = "([])"
        // Expected: true
        assert_eq!(Solution::is_valid("([])".to_string()), true);
    }

    #[test]
    fn example_5() {
        // Input: s = "([)]"
        // Expected: false
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }
}
