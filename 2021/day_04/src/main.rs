use std::error::Error;
use std::io::{self, prelude::*};

pub type SquareIndex = u8;

pub const TOTAL_ROWS: SquareIndex = 5;
pub const TOTAL_COLUMNS: SquareIndex = 5;
pub const TOTAL_SQUARES: SquareIndex = TOTAL_ROWS * TOTAL_COLUMNS;

#[derive(Clone, Copy, Debug)]
pub struct Square {
    value: i32,
    marked: bool,
}

pub struct BingoBoard {
    board: [Square; TOTAL_SQUARES as usize],
    last_num: i32,
}

impl BingoBoard {
    pub fn new(values: &[i32; TOTAL_SQUARES as usize]) -> BingoBoard {
        let mut board = [Square {
            value: 0,
            marked: false,
        }; TOTAL_SQUARES as usize];

        for (ix, &value) in values.iter().enumerate() {
            board[ix] = Square {
                value,
                marked: false,
            };
        }

        BingoBoard { board, last_num: 0 }
    }

    pub fn mark_square(&mut self, num: i32) {
        let square_index = self.get_square_index(num);
        self.board[square_index as usize].marked = true;
        self.last_num = num;
    }

    pub fn won(&self) -> bool {
        let square_index = self.get_square_index(self.last_num);

        let mut won = self
            .get_row_indices(square_index)
            .iter()
            .all(|&ix| self.board[ix as usize].marked)
            || self
                .get_column_indices(square_index)
                .iter()
                .all(|&ix| self.board[ix as usize].marked);

        return won;
    }

    fn get_square_index(&self, num: i32) -> SquareIndex {
        self.board
            .iter()
            .position(|&square| square.value == num)
            .unwrap() as SquareIndex
    }

    fn get_row_indices(&self, start_index: SquareIndex) -> Vec<SquareIndex> {
        let mut indices = vec![];

        let min_column_ix = ((start_index / TOTAL_COLUMNS) * TOTAL_COLUMNS);

        for ix in 0..TOTAL_COLUMNS {
            let row_ix = ((start_index - min_column_ix + ix) % (TOTAL_COLUMNS)) + min_column_ix;

            indices.push(row_ix);
        }

        indices
    }

    fn get_column_indices(&self, start_index: SquareIndex) -> Vec<SquareIndex> {
        let mut indices = vec![];

        for ix in 0..TOTAL_ROWS {
            let col_ix = (start_index + (ix * TOTAL_ROWS)) % TOTAL_SQUARES;

            indices.push(col_ix);
        }

        indices
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let parsed: Result<Vec<u32>, Box<dyn Error>> =
        lines.map(|line| Ok(line?.parse::<u32>()?)).collect();

    match parsed {
        Ok(p) => {
            for n in p {
                println!("{}", n);
            }
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

#[test]
fn test_calculate_row_complete() {
    let mut board = BingoBoard::new(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ]);
    board.mark_square(1);
    assert_eq!(board.won(), false);
    board.mark_square(2);
    assert_eq!(board.won(), false);
    board.mark_square(3);
    assert_eq!(board.won(), false);
    board.mark_square(4);
    assert_eq!(board.won(), false);
    board.mark_square(5);
    assert_eq!(board.won(), true);
}

#[test]
fn test_calculate_row_complete2() {
    let mut board = BingoBoard::new(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ]);
    board.mark_square(25);
    assert_eq!(board.won(), false);
    board.mark_square(24);
    assert_eq!(board.won(), false);
    board.mark_square(23);
    assert_eq!(board.won(), false);
    board.mark_square(22);
    assert_eq!(board.won(), false);
    board.mark_square(21);
    assert_eq!(board.won(), true);
}

#[test]
fn test_calculate_column_complete() {
    let mut board = BingoBoard::new(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ]);
    board.mark_square(1);
    assert_eq!(board.won(), false);
    board.mark_square(6);
    assert_eq!(board.won(), false);
    board.mark_square(11);
    assert_eq!(board.won(), false);
    board.mark_square(16);
    assert_eq!(board.won(), false);
    board.mark_square(21);
    assert_eq!(board.won(), true);
}

#[test]
fn test_calculate_column_complete2() {
    let mut board = BingoBoard::new(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ]);
    board.mark_square(5);
    assert_eq!(board.won(), false);
    board.mark_square(10);
    assert_eq!(board.won(), false);
    board.mark_square(15);
    assert_eq!(board.won(), false);
    board.mark_square(20);
    assert_eq!(board.won(), false);
    board.mark_square(25);
    assert_eq!(board.won(), true);
}

#[test]
fn test_row_indices() {
    let mut board = BingoBoard::new(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ]);
    assert_eq!(board.get_row_indices(0), vec![0, 1, 2, 3, 4]);
    assert_eq!(board.get_row_indices(1), vec![1, 2, 3, 4, 0]);
    assert_eq!(board.get_row_indices(2), vec![2, 3, 4, 0, 1]);
    assert_eq!(board.get_row_indices(3), vec![3, 4, 0, 1, 2]);
    assert_eq!(board.get_row_indices(4), vec![4, 0, 1, 2, 3]);
    assert_eq!(board.get_row_indices(24), vec![24, 20, 21, 22, 23]);
    assert_eq!(board.get_row_indices(23), vec![23, 24, 20, 21, 22]);
    assert_eq!(board.get_row_indices(22), vec![22, 23, 24, 20, 21]);
    assert_eq!(board.get_row_indices(21), vec![21, 22, 23, 24, 20]);
    assert_eq!(board.get_row_indices(20), vec![20, 21, 22, 23, 24]);
}

#[test]
fn test_col_indices() {
    let mut board = BingoBoard::new(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ]);
    assert_eq!(board.get_column_indices(0), vec![0, 5, 10, 15, 20]);
    assert_eq!(board.get_column_indices(1), vec![1, 6, 11, 16, 21]);
    assert_eq!(board.get_column_indices(2), vec![2, 7, 12, 17, 22]);
    assert_eq!(board.get_column_indices(3), vec![3, 8, 13, 18, 23]);
    assert_eq!(board.get_column_indices(4), vec![4, 9, 14, 19, 24]);
    assert_eq!(board.get_column_indices(24), vec![24, 4, 9, 14, 19]);
    assert_eq!(board.get_column_indices(23), vec![23, 3, 8, 13, 18]);
    assert_eq!(board.get_column_indices(22), vec![22, 2, 7, 12, 17]);
    assert_eq!(board.get_column_indices(21), vec![21, 1, 6, 11, 16]);
    assert_eq!(board.get_column_indices(20), vec![20, 0, 5, 10, 15]);
}
