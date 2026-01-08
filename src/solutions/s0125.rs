/**
 * 125. Valid Palindrome
 *
 * A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers. Given a string s, return true if it is a palindrome, or false otherwise.
 *
 * Example 1:
 * Input: s = "A man, a plan, a canal: Panama"
 * Output: true
 *
 * Example 2:
 * Input: s = "race a car"
 * Output: false
 *
 * Example 3:
 * Input: s = " "
 * Output: true
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Is not the optimal solution,
        // but I want to try a zero allocation solution wihout the two pointer approach
        let mut bytes = s.into_bytes();
        let mut len = 0;
        for i in 0..bytes.len() {
            if bytes[i].is_ascii_alphanumeric() {
                let byte = bytes[i].to_ascii_lowercase();
                bytes[len] = byte;
                len += 1;
            }
        }
        let bytes = &bytes[0..len];
        for i in 0..len / 2 {
            if bytes[i] != bytes[len - 1 - i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: s = "A man, a plan, a canal: Panama"
        // Expected: true
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn example_2() {
        // Input: s = "race a car"
        // Expected: false
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn example_3() {
        // Input: s = " "
        // Expected: true
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}
