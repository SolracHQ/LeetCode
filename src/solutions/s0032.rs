/**
* 32. Longest Valid Parentheses
*
* Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.

*
* Example 1:
* Input: s = "(()"
* Output: 2
*
* Example 2:
* Input: s = ")()())"
* Output: 4
*
* Example 3:
* Input: s = ""
* Output: 0

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut result = 0;
        let mut stack = vec![0];
        let bytes = s.as_bytes();
        for i in 0..s.len() {
            match bytes[i] {
                // If the character is an opening parenthesis, push its index onto the stack
                b'(' => stack.push(i + 1),
                b')' => {
                    // If the character is a closing parenthesis, pop the top element from the stack
                    stack.pop();
                    if stack.is_empty() {
                        // If the stack is empty, push the current index onto the stack
                        stack.push(i + 1);
                    } else {
                        // Update the result with the maximum length of valid parentheses substring found so far
                        result = result.max(i + 1 - stack.last().unwrap());
                    }
                }
                _ => {}
            }
        }
        result as _
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: s = "(()"
        // Expected: 2
        assert_eq!(2, Solution::longest_valid_parentheses("(()".to_string()));
    }

    #[test]
    fn example_2() {
        // Input: s = ")()())"
        // Expected: 4
        let s = ")()())".to_string();
        assert_eq!(4, Solution::longest_valid_parentheses(s));
    }

    #[test]
    fn example_3() {
        // Input: s = ""
        // Expected: 0
        assert_eq!(0, Solution::longest_valid_parentheses("".to_string()));
    }
}
