// https://leetcode.com/problems/string-to-integer-atoi/

use crate::common::Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();
        if s.len() == 0 {
            return 0;
        }
        let is_negative = s.as_bytes()[0] as char == '-';
        let s = if s.as_bytes()[0] as char == '+' || is_negative {
            unsafe {std::mem::transmute(&s.as_bytes()[1..])}
        } else {
            s
        };
        let s = s
            .chars()
            .take_while(|c| {
                !c.is_alphabetic() && !c.is_whitespace() && *c != '.' && *c != '-' && *c != '+'
            })
            .collect::<String>();
        if s.len() == 0 {
            return 0;
        }
        if is_negative {
            return format!("-{}", s).parse().unwrap_or(i32::MIN);
        }
        s.parse().unwrap_or(i32::MAX)
    }
}

mod test {
    use crate::common::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::my_atoi("42".to_owned()), 42)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::my_atoi("       -42".to_owned()), -42)
    }
    #[test]
    fn example_3() {
        assert_eq!(Solution::my_atoi("4193 with words".to_owned()), 4193)
    }
    #[test]
    fn example_4() {
        assert_eq!(Solution::my_atoi("-91283472332".to_owned()), -2147483648)
    }
    #[test]
    fn example_5() {
        assert_eq!(Solution::my_atoi("words and 987".to_owned()), 0)
    }
    #[test]
    fn example_6() {
        assert_eq!(Solution::my_atoi("3.14159".to_owned()), 3)
    }
    #[test]
    fn example_7() {
        assert_eq!(Solution::my_atoi("+-12".to_owned()), 0)
    }
    #[test]
    fn example_8() {
        assert_eq!(Solution::my_atoi("00000-42a1234".to_owned()), 0)
    }
}
