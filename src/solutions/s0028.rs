/**
* 28. Find the Index of the First Occurrence in a String
*
* Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

*
* Example 1:
* Input: haystack = "sadbutsad", needle = "sad"
* Output: 0
*
* Example 2:
* Input: haystack = "leetcode", needle = "leeto"
* Output: -1

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|r| r as i32).unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: haystack = "sadbutsad", needle = "sad"
        // Expected: 0
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
    }

    #[test]
    fn example_2() {
        // Input: haystack = "leetcode", needle = "leeto"
        // Expected: -1
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }
}
