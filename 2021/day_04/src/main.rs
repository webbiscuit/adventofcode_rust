use std::error::Error;
use std::fmt;
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

#[derive(Clone, Copy, Debug)]

pub struct BingoBoard {
    board: [Square; TOTAL_SQUARES as usize],
    last_num: i32,
}

impl fmt::Display for BingoBoard {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Bingo Board:")?;
        for row in 0..TOTAL_ROWS {
            for col in 0..TOTAL_COLUMNS {
                let ix: usize = (row * TOTAL_COLUMNS + col) as usize;
                let square = &self.board[ix];

                write!(
                    f,
                    "{0: >3}",
                    if square.marked {
                        "x".to_string()
                    } else {
                        square.value.to_string()
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
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

        if let Some(square_index) = square_index {
            self.board[square_index as usize].marked = true;
            self.last_num = num;
        }
    }

    pub fn won(&self) -> bool {
        let square_index = self.get_square_index(self.last_num);

        if let Some(square_index) = square_index {
            let won = self
                .get_row_indices(square_index)
                .iter()
                .all(|&ix| self.board[ix as usize].marked)
                || self
                    .get_column_indices(square_index)
                    .iter()
                    .all(|&ix| self.board[ix as usize].marked);

            won
        } else {
            false
        }
    }

    fn get_square_index(&self, num: i32) -> Option<SquareIndex> {
        self.board
            .iter()
            .position(|&square| square.value == num)
            .map(|ix| ix as SquareIndex)
    }

    fn get_row_indices(&self, start_index: SquareIndex) -> Vec<SquareIndex> {
        let mut indices = vec![];

        let min_column_ix = (start_index / TOTAL_COLUMNS) * TOTAL_COLUMNS;

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

    pub fn get_score(&self) -> i32 {
        let mut score = 0;

        for square in self.board.iter() {
            if !square.marked {
                score += square.value;
            }
        }

        score * self.last_num
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let called_numbers = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut cards: Vec<Vec<i32>> = vec![];
    let mut current_card: Vec<i32> = vec![];

    for line in lines {
        let line = line?;

        if line.is_empty() {
            if !current_card.is_empty() {
                cards.push(current_card);
            }
            current_card = vec![];
        }

        let nums = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        current_card.extend(nums);
    }

    if !current_card.is_empty() {
        cards.push(current_card);
    }

    let mut bingo_boards: Vec<BingoBoard> = vec![];

    for card in cards {
        let board = BingoBoard::new(&card.try_into().expect("slice with incorrect length"));
        bingo_boards.push(board);
    }

    let mut winner: Option<BingoBoard> = None;

    'outer: for n in &called_numbers {
        for board in &mut bingo_boards {
            board.mark_square(*n);

            if board.won() {
                winner = Some(*board);
                break 'outer;
            }
        }
    }

    let mut loser: Option<BingoBoard> = None;
    for n in &called_numbers {
        bingo_boards.iter_mut().for_each(|board| {
            board.mark_square(*n);
        });

        if bingo_boards.len() == 1 && bingo_boards[0].won() {
            loser = Some(bingo_boards[0]);
            break;          
        }

        bingo_boards = bingo_boards
            .iter()
            .filter(|board| !board.won())
            .cloned()
            .collect();
    }

    if let Some(winner) = winner {
        // println!("{}", winner);
        println!("Bingo! Final score: {}", winner.get_score());
    } else {
        println!("No winner");
    }

    if let Some(loser) = loser {
        println!("Bongo! Final score of final card: {}", loser.get_score());
    } else {
        println!("No loser");
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
    let board = BingoBoard::new(&[
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
    let board = BingoBoard::new(&[
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
