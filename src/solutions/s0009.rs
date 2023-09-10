/**
 * # 9. Palindrome Number - https://leetcode.com/problems/palindrome-number/
 * Given an integer x, return true if x is a palindrome, and false otherwise.
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {return false;}
        let s = x.to_string();
        s == s.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::is_palindrome(121))
    }
    #[test]
    fn example_2() {
        assert!(!Solution::is_palindrome(-121))
    }
    #[test]
    fn example_3() {
        assert!(!Solution::is_palindrome(10))
    }
}