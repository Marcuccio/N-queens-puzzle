use std::io;
use std::cmp;
use std::fmt;

struct Chessboard {
    board: Vec<Vec<u16>>,
}

fn main() {

    println!("Please input the number of queens.");

    let mut queens_count = String::new();

    io::stdin().read_line(&mut queens_count).expect("Failed to read line");

    let n: usize = match queens_count.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    let mut chessboard = Chessboard::new(n);
}

impl Chessboard {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        Chessboard {
            board: vec![vec![0; size]; size]
        }
    }

    #[allow(dead_code)]
    pub fn add_queen(&mut self, row: usize, col: usize) {
        assert!(row < self.board.len() && col < self.board.len(), "Coordinates out of bound");

        self.board[row][col] = self.board[row][col] + 1;

        let mut l_start_diagonal_row: usize = cmp::max(row - col, 0);
        let mut l_start_diagonal_col: usize = col - (row - l_start_diagonal_row);

        let r_start_diagonal_offset = cmp::min(row, self.board.len() - col - 1);
        let mut r_start_diagonal_row: usize = row - r_start_diagonal_offset;
        let mut r_start_diagonal_col: usize = col + r_start_diagonal_offset;

        let mut index: usize = 0;

        println!("left-diag: {},{}; right-diag: {},{}", l_start_diagonal_row, l_start_diagonal_col, r_start_diagonal_row, r_start_diagonal_col);

        while index < self.board.len() {
            /* The [row][col] position counter is modified above this loop.
            ** Here, in this loop, we want to avoid to update the [row][col] again and again
            ** during the column, row, diagonal counter-updating.
            */
            if index != col {
                self.board[row][index] = self.board[row][index] + 1;
            }

            if index != row {
                self.board[index][col] = self.board[index][col] + 1;
            }

            if l_start_diagonal_row < self.board.len() && l_start_diagonal_col < self.board.len()
                    && l_start_diagonal_row != row && l_start_diagonal_col != col {

                self.board[l_start_diagonal_row][l_start_diagonal_col] =
                        self.board[l_start_diagonal_row][l_start_diagonal_col] + 1;

                l_start_diagonal_row = l_start_diagonal_row + 1;
                l_start_diagonal_col = l_start_diagonal_col + 1;
            }

            if r_start_diagonal_row < self.board.len() && r_start_diagonal_col > 0
                    && r_start_diagonal_row != row && r_start_diagonal_col != col {

                self.board[r_start_diagonal_row][r_start_diagonal_col] =
                        self.board[r_start_diagonal_row][r_start_diagonal_col] + 1;

                r_start_diagonal_row = r_start_diagonal_row + 1;
                r_start_diagonal_col = r_start_diagonal_col - 1;
            }

            index = index + 1;
        }
    }

    #[allow(dead_code)]
    pub fn remove_queen(&mut self, row: usize, col: usize) {
        assert!(row < self.board.len() && col < self.board.len(), "Coordinates out of bound");

        self.board[row][col] = self.board[row][col] - 1;

        let mut l_start_diagonal_row: usize = cmp::max(row - col, 0);
        let mut l_start_diagonal_col: usize = col - (row - l_start_diagonal_row);

        let r_start_diagonal_offset = cmp::min(row, self.board.len() - col - 1);
        let mut r_start_diagonal_row: usize = row - r_start_diagonal_offset;
        let mut r_start_diagonal_col: usize = col + r_start_diagonal_offset;

        let mut index: usize = 0;

        while index < self.board.len() {
            /* The [row][col] posit`ion counter is modified above this loop.
            ** Here, in this loop, we want to avoid to update the [row][col] again and again
            ** during the column, row, diagonal counter-updating.
            */
            if index != col {
                self.board[row][index] = self.board[row][index] - 1;
            }

            if index != row {
                self.board[index][col] = self.board[index][col] - 1;
            }

            if l_start_diagonal_row < self.board.len() && l_start_diagonal_col < self.board.len()
                    && l_start_diagonal_row != row && l_start_diagonal_col != col {

                self.board[l_start_diagonal_row][l_start_diagonal_col] =
                        self.board[l_start_diagonal_row][l_start_diagonal_col] - 1;

                l_start_diagonal_row = l_start_diagonal_row + 1;
                l_start_diagonal_col = l_start_diagonal_col + 1;
            }

            if r_start_diagonal_row < self.board.len() && r_start_diagonal_col > 0
                    && r_start_diagonal_row != row && r_start_diagonal_col != col {

                self.board[r_start_diagonal_row][r_start_diagonal_col] =
                        self.board[r_start_diagonal_row][r_start_diagonal_col] - 1;

                r_start_diagonal_row = r_start_diagonal_row + 1;
                r_start_diagonal_col = r_start_diagonal_col - 1;
            }

            index = index + 1;
        }
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing
        // and create a reference to `vec`.
        let vec = &self.board;

        write!(f, "[\n")?;

        // Iterate over `vec` in `v` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 { write!(f, "\n")?; }
            write!(f, "{:?}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value
        write!(f, "\n]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_queen_1() {
        let mut chessboard = Chessboard::new(4);
        chessboard.add_queen(3,2);
        assert_eq!(chessboard.board, [[0, 0, 1, 0], [1, 0, 1, 0], [0, 1, 1, 1], [1, 1, 1, 1]]);
        // println!("{:}", chessboard);
    }

    #[test]
    fn add_remove_queen_1() {
        let mut chessboard = Chessboard::new(4);
        chessboard.add_queen(3,2);
        assert_eq!(chessboard.board, [[0, 0, 1, 0], [1, 0, 1, 0], [0, 1, 1, 1], [1, 1, 1, 1]]);
        chessboard.remove_queen(3, 2);
        assert_eq!(chessboard.board, [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
        // println!("{:}", chessboard);
    }
}
