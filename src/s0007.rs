//https://leetcode.com/problems/reverse-integer/
use crate::common::Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let negative = x < 0;
        let mut soulution = i32::abs(x)
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0);
        if negative {
            soulution *= -1
        }
        soulution
    }
}

mod test {
    use crate::common::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::reverse(123),
            321
        )
    }
    #[test]
    fn example_2() {
        assert_eq!(
            Solution::reverse(-123),
            -321
        )
    }
    #[test]
    fn example_3() {
        assert_eq!(
            Solution::reverse(120),
            21
        )
    }
}