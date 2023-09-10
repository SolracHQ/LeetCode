/**
 * 118. Pascal's Triangle
 * Given an integer numRows, return the first numRows of Pascal's triangle.
 * In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
 */
struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        if num_rows <= 0 {
            return vec![];
        }
        let mut result = Vec::with_capacity(num_rows);
        
        for i in 0..num_rows {
            result.push(vec![1; i + 1]);
            for j in 1..result[i].len()-1 {
                result[i][j] = result[i-1][j-1] + result[i-1][j];
            }
        }
        
        result
    }
}

mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::generate(5),
            vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]]
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::generate(1),
            vec![vec![1]]
        )
    }
}