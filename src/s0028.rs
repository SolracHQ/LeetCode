/*
 * 28. Find the Index of the First Occurrence in a String - https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
 * 
 * Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
*/
struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|r| r as i32).unwrap_or(-1)
    }
}

mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::str_str(format!("sadbutsad"), format!("sad")),
            0
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::str_str(format!("leetcode"), format!("leeto")),
            -1
        )
    }
}