/**
 * 36. Valid Sudoku
 * Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
 *
 * Each row must contain the digits 1-9 without repetition.
 * Each column must contain the digits 1-9 without repetition.
 * Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
 * Note:
 *
 * A Sudoku board (partially filled) could be valid but is not necessarily solvable.
 * Only the filled cells need to be validated according to the mentioned rules.
 */
struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j].is_ascii_digit() && !Solution::check_point((i,j), &board){
                    return false;
                } 
            }
        }
        true
    }
    fn check_point(selected: (usize, usize), board: &[Vec<char>]) -> bool {
        let (x, y) = selected;
        let value = board[x][y];
        // Check row
        for i in 0..9 {
            if board[x][i] == value && (x, i) != selected {
                return false;
            }
        }
        // Check column
        for i in 0..9 {
            if board[i][y] == value && (i, y) != selected {
                return false;
            }
        }
        // Check subgrid
        let subgrid_x = x / 3 * 3;
        let subgrid_y = y / 3 * 3;
        for i in subgrid_x..subgrid_x + 3 {
            for j in subgrid_y..subgrid_y + 3 {
                if board[i][j] == value && (i, j) != selected {
                    return false;
                }
            }
        }
        true
    }
}

mod test {
    use super::*;

    #[test]
    fn example_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true)
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false)
    }
}
