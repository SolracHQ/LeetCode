/**
* 58. Length of Last Word
*
* Given a string s consisting of words and spaces, return the length of the last word in the string.

*
* Example 1:
* Input: s = "Hello World"
* Output: 5
*
* Example 2:
* Input: s = "   fly me   to   the moon  "
* Output: 4
*
* Example 3:
* Input: s = "luffy is still joyboy"
* Output: 6

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut indices = (0, s.len() - 1);
        while s.as_bytes()[indices.1] == b' ' && indices.1 > 0 {
            indices.1 -= 1;
        }
        indices.0 = indices.1;
        while s.as_bytes()[indices.0] != b' ' && indices.0 > 0 {
            indices.0 -= 1;
        }
        if indices.0 == 0 && s.as_bytes()[indices.0] != b' ' {
            indices.1 += 1
        }
        (indices.1 - indices.0) as _
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: s = "Hello World"
        // Expected: 5
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn example_2() {
        // Input: s = "   fly me   to   the moon  "
        // Expected: 4
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn example_3() {
        // Input: s = "luffy is still joyboy"
        // Expected: 6
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
