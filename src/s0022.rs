/**
 * # 22. Generate Parentheses - https://leetcode.com/problems/generate-parentheses/
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        stack.push(("".to_string(), n, n));
        while !stack.is_empty() {
            let (current, left, right) = stack.pop().unwrap();
            if left == 0 && right == 0 {
                result.push(current.clone());
            }
            if left > 0 {
                stack.push((current.clone() + "(", left - 1, right));
            }
            if right > left {
                stack.push((current + ")", left, right - 1));
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::Sortable;
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::generate_parenthesis(3).sorted(),
            vec!["((()))".to_string(), "(()())".to_string(), "(())()".to_string(), "()(())".to_string(), "()()()".to_string()].sorted()
        )
    }
}