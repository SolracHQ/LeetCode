/**
* 13. Roman to Integer
*
* Given a roman numeral, convert it to an integer.

*
* Example 1:
* Input: s = "III"
* Output: 3
*
* Example 2:
* Input: s = "LVIII"
* Output: 58
*
* Example 3:
* Input: s = "MCMXCIV"
* Output: 1994

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
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
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: s = "III"
        // Expected: 3
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
    }

    #[test]
    fn example_2() {
        // Input: s = "LVIII"
        // Expected: 58
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    }

    #[test]
    fn example_3() {
        // Input: s = "MCMXCIV"
        // Expected: 1994
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
