// https://leetcode.com/problems/palindrome-number/

use crate::common::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {return false;}
        if x == 0 {return true;}
        let size = f64::log10(x as f64) as u32 + 1;
        for i in 0..=(size/2) {
            if Solution::digit(x, i) != Solution::digit(x, size - i - 1){
                return false;
            }
        }
        true
    }

    fn digit(num: i32, pos: u32) -> u32 {
        (num as u32/Solution::pos(pos)) % 10
    }

    fn pos(x: u32) -> u32 {
        match x {
            0  => 1,
            1  => 10,
            2  => 100,
            3  => 1000,
            4  => 10000,
            5  => 100000,
            6  => 1000000,
            7  => 10000000,
            8  => 100000000,
            9  => 1000000000,
            _ => 0
        }
    }
}

mod test {
    use crate::common::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::is_palindrome(121))
    }
    #[test]
    fn example_2() {
        assert!(!Solution::is_palindrome(-121))
    }
    #[test]
    fn example_3() {
        assert!(!Solution::is_palindrome(10))
    }
}