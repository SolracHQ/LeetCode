// https://leetcode.com/problems/zigzag-conversion/
use crate::common::Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() <= num_rows as usize || num_rows == 1 {
            return s;
        }
        let mut result = String::with_capacity(s.len());
        let mut rows = Vec::<Vec<u8>>::with_capacity(num_rows as usize);
        for _ in 0..num_rows{
            rows.push(vec![]);
        }
        let mut index = 0;
        let mut step = 1;
        for ch in s.as_bytes() {
            rows[index].push(*ch);
            if index == 0 {
                step = 1;
            } else if index as i32 == num_rows - 1 {
                step = -1;
            }
            index = (index as i32 + step) as usize; 
        }
        for row in rows {
            result.push_str(std::str::from_utf8(&row).unwrap());
        }
        result
    }
}

mod test {
    use crate::common::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            "PAHNAPLSIIGYIR",
            &Solution::convert("PAYPALISHIRING".to_owned(), 3)
        )
    }
    #[test]
    fn example_2() {
        assert_eq!(
            "PINALSIGYAHRPI",
            &Solution::convert("PAYPALISHIRING".to_owned(), 4)
        )
    }
    #[test]
    fn example_3() {
        assert_eq!(
            "A",
            &Solution::convert("A".to_owned(), 1)
        )
    }
}