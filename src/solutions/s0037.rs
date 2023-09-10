/**
 * 37. Sudoku Solver
 * Write a program to solve a Sudoku puzzle by filling the empty cells.
 *
 * A sudoku solution must satisfy all of the following rules:
 *
 * Each of the digits 1-9 must occur exactly once in each row.
 * Each of the digits 1-9 must occur exactly once in each column.
 * Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
 * The'.' character indicates empty cells.
 */
struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let to_fill = board.iter().enumerate().fold(vec![], |mut acc, (i, row)| {
            row.iter().enumerate().for_each(|(j, element)| {
                if '.' == *element {
                    acc.push((i, j));
                }
            });
            acc
        });

        let mut index = 0;
        // Iterate over all the cells that need to be filled
        'out: while index < to_fill.len() {
            // Select the current cell
            let selected = to_fill[index];
            // Try all possible values for the current cell
            for possibility in ('1').max((board[selected.0][selected.1] as u8 + 1) as char)..='9'
            {
                board[selected.0][selected.1] = possibility as char;
                // Check if the current possibility is valid
                if Self::check_point(selected, &board) {
                    index += 1;
                    continue 'out;
                }
            }
            // No valid value was found for the current cell, backtrack
            board[selected.0][selected.1] = '.';
            index -= 1;
        }
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
        let mut board = vec![
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
        let solution = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, solution);
    }
}
