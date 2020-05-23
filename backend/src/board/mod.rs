mod piece;
mod position;

use std::fmt;
use std::ops::{Index, IndexMut};

use nalgebra::{MatrixN, U10};

use piece::Piece;
use crate::board::position::Position;

const MATRIX_SIZE: usize = 10;

pub struct Board {
    cells: MatrixN<Piece, U10>,
}

impl Board {
    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize, usize) -> Piece
    {
        let cells = MatrixN::<Piece, U10>::from_fn(f);
        Self { cells}
    }
    fn position_from_index(index: usize) -> Position {
        (index / MATRIX_SIZE, index % MATRIX_SIZE).into()
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cells)
    }
}

impl Index<usize> for Board {
    type Output = Piece;

    fn index(&self, idx: usize) -> &Self::Output {
        self.cells.index(idx)
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Piece;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        self.cells.index(pos)
    }
}

impl Index<Position> for Board {
    type Output = Piece;

    fn index(&self, pos: Position) -> &Self::Output {
        self.cell.index(pos.into())
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.cells.index_mut(idx)
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        self.cells.index_mut(pos)
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        self.cells.index_mut(pos.into())
    }
}


#[test]
fn test_board_creation() {
    let board = Board::from_fn(|x,y| {
        match x {
            0..=3 if (x + y) % 2 == 1 => Piece::BlackPawn,
            6..=9 if (x + y) % 2 == 1 => Piece::WhitePawn,
            _ => Piece::Empty
        }
    });
    // FIXME: write a meaningful test
    assert!(false);
}

#[test]
fn test_position_from_index() {
    assert_eq!(Board::position_from_index(9), Position((0,9)));
    assert_eq!(Board::position_from_index(15), Position((1,5)));
    assert_eq!(Board::position_from_index(10), Position((1,0)));
    assert_eq!(Board::position_from_index(19), Position((1,9)));
}

// TODO: test board indexing
