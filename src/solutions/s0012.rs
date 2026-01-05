/**
* 12. Integer to Roman
*
* Given an integer, convert it to a roman numeral.

*
* Example 1:
* Input: num = 3749
* Output: "MMMDCCXLIX"
*
* Example 2:
* Input: num = 58
* Output: "LVIII"
*
* Example 3:
* Input: num = 1994
* Output: "MCMXCIV"

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        const UNIT: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        const DECENT: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        const CENTENT: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];

        let mut result = String::new();
        result.push_str(THOUSANDS[(num / 1000) as usize]);
        result.push_str(CENTENT[(num as usize / 100) % 10]);
        result.push_str(DECENT[(num as usize / 10) % 10]);
        result.push_str(UNIT[num as usize % 10]);
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        // Input: num = 3749
        // Expected: "MMMDCCXLIX"
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX".to_string());
    }

    #[test]
    fn example_2() {
        // Input: num = 58
        // Expected: "LVIII"
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn example_3() {
        // Input: num = 1994
        // Expected: "MCMXCIV"
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
