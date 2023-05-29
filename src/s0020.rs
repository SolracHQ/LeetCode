/**
 * # 20. Valid Parentheses - https://leetcode.com/problems/valid-parentheses
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 * An input string is valid if:
 * - Open brackets must be closed by the same type of brackets.
 * - Open brackets must be closed in the correct order.
 *
 * Every close bracket has a corresponding open bracket of the same type.
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
struct Stack {
    data: Vec<u8>,
}

#[cfg(test)]
impl Stack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, item: u8) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<u8> {
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Stack::new();
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
        assert!(Solution::is_valid("()".to_string()))
    }

    #[test]
    fn example_2() {
        assert!(Solution::is_valid("()[]{}".to_string()))
    }

    #[test]
    fn example_3() {
        assert!(!Solution::is_valid("(}".to_string()))
    }
}