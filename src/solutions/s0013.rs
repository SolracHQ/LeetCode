// https://leetcode.com/problems/roman-to-integer/
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn lookup(ch: char) -> i32 {
            match ch {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        let mut max = 0;
        let mut result = 0;
        for ch in s.chars().rev() {
            if lookup(ch) < max {
                result -= lookup(ch);
            } else {
                max = lookup(ch);
                result += max;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58)
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994)
    }
}
