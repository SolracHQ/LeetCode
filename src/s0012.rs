use crate::common::Solution;

/*
    Lookup tables
*/
const UNIT: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
const DECENT: [&str; 10] = ["","X","XX","XXX","XL","L","LX","LXX","LXXX","XC"];
const CENTENT: [&str; 10] = ["","C","CC","CCC","CD","D","DC","DCC","DCCC","CM"];
const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::new();
        result.push_str(THOUSANDS[(num/1000) as usize]);
        result.push_str(CENTENT[(num as usize / 100) % 10]);
        result.push_str(DECENT[(num as usize / 10) % 10]);
        result.push_str(UNIT[num as usize % 10]);
        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string())
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string())
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string())
    }
}
