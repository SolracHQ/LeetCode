/**
* 9. Palindrome Number
*
* Given an integer x, return true if x is a palindrome, and false otherwise.

*
* Example 1:
* Input: x = 121
* Output: true
*
* Example 2:
* Input: x = -121
* Output: false
*
* Example 3:
* Input: x = 10
* Output: false

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s = x.to_string();
        s == s.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: x = 121
        // Expected: true
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn example_2() {
        // Input: x = -121
        // Expected: false
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn example_3() {
        // Input: x = 10
        // Expected: false
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
