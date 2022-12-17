// https://leetcode.com/problems/longest-palindromic-substring/

use std::string::String;

use crate::common::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        for size in (2..=s.len()).rev() {
            for start in 0..=(s.len() - size) {
                if Self::is_palindromic(&s.as_bytes()[start..size+start]) {
                    return String::from_utf8_lossy(&s.as_bytes()[start..size+start]).to_string();
                }
            }
        }
        String::from_utf8_lossy(&[s.as_bytes()[0]]).to_string()
    }

    fn is_palindromic(s: &[u8]) -> bool {
        for i in 0..=(s.len()/2) {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

mod test {
    use crate::{s0005, common::Solution};


    #[test]
    fn is_palindromic_test() {
        assert!(Solution::is_palindromic("bb".as_bytes()))
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_owned()),
            "bab"
        )
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_owned()),
            "bb"
        )
    }
    #[test]
    fn example_3() {
        assert_eq!(
            Solution::longest_palindrome("abb".to_owned()),
            "bb"
        )
    }
}