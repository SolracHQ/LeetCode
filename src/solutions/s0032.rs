/*
 * 32. Longest Valid Parentheses - https://leetcode.com/problems/longest-valid-parentheses/
 * Given a string containing just the characters '(' and ')', return the length of the longest 
 * valid (well-formed) parentheses substring.
 */
struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut result = 0;
        let mut stack = vec![0];
        let bytes = s.as_bytes();
        for i in 0..s.len()  {
            match bytes[i] {
                // If the character is an opening parenthesis, push its index onto the stack
                b'(' => { stack.push(i + 1) }
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
                _ =>{}
            }
        }
        result as _
    }
}


mod test{
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_valid_parentheses(format!("(()")),
            2
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_valid_parentheses(format!(")()())")),
            4
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::longest_valid_parentheses(format!("")),
            0
        )
    }
}