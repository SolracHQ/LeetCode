/**
 * # 6. Zigzag Conversion - https://leetcode.com/problems/zigzag-conversion/
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number
 * of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 */
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // If the string is shorter than or equal to the number of rows or the number of rows is 1,
        // it can't form any zigzag pattern, so just return the original string.
        if s.len() <= num_rows as usize || num_rows == 1 {
            return s;
        }

        // Create a string with the same capacity as the original string.
        let mut result = String::with_capacity(s.len());

        // Create a vector of vectors to store each character at its corresponding row.
        let mut rows = vec![Vec::new(); num_rows as usize];

        let mut index = 0;
        let mut step = 1;

        // Traverse the input string and store each character in its corresponding row based on the zigzag pattern.
        for ch in s.as_bytes() {
            rows[index].push(*ch as char);
            // If we are at the top row, we need to move down to the next row.
            if index == 0 {
                step = 1;
                // If we are at the bottom row, we need to move up to the next row.
            } else if index as i32 == num_rows - 1 {
                step = -1;
            }
            // Update the row index based on the current step direction.
            index = (index as i32 + step) as usize;
        }

        // Flatten the vector of rows into a single string and return it.
        for row in rows {
            result.extend(row.into_iter());
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

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