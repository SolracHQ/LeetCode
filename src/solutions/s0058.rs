/**
 * 58. Length of Last Word
 * Given a string s consisting of words and spaces, return the length of the last word in the string.
 * A word is a maximal substring consisting of non-space characters only.
 */
struct Solution;

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

mod test {
    use crate::solutions::s0058::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::length_of_last_word("Hello World".into()),
            5
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".into()),
            4
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".into()),
            6
        )
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::length_of_last_word("y".into()),
            1
        )
    }

    #[test]
    fn example5() {
        assert_eq!(
            Solution::length_of_last_word("day".into()),
            3
        )
    }

    #[test]
    fn example6() {
        assert_eq!(
            Solution::length_of_last_word(" y".into()),
            1
        )
    }
}