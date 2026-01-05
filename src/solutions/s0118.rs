/**
* 118. Pascal's Triangle
*
* Given an integer numRows, return the first numRows of Pascal's triangle.

*
* Example 1:
* Input: numRows = 5
* Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
*
* Example 2:
* Input: numRows = 1
* Output: [[1]]

*/
#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows <= 0 {
            return vec![];
        }
        let num_rows = num_rows as usize;
        let mut result = Vec::with_capacity(num_rows);

        for i in 0..num_rows {
            result.push(vec![1; i + 1]);
            for j in 1..result[i].len() - 1 {
                result[i][j] = result[i - 1][j - 1] + result[i - 1][j];
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
        // Input: numRows = 5
        // Expected: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn example_2() {
        // Input: numRows = 1
        // Expected: [[1]]
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }
}
