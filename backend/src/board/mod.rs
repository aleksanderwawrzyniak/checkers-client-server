mod cell;
mod piece;
mod position;
mod turn;

use std::fmt;
use std::ops::{Index, IndexMut};

use nalgebra::base::iter::MatrixIter;
use nalgebra::{ArrayStorage, MatrixN, U10};

use crate::board::cell::Cell;
use crate::board::position::Position;
use crate::board::turn::Move;
use piece::{Color as PieceColor, Piece};

const MATRIX_SIZE: usize = 10;

pub type BoardIter<'a> = MatrixIter<'a, Piece, U10, U10, ArrayStorage<Piece, U10, U10>>;

pub struct Board {
    cells: MatrixN<Piece, U10>,
}

impl Board {
    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize, usize) -> Piece,
    {
        let cells = MatrixN::<Piece, U10>::from_fn(f);
        Self { cells }
    }

    pub fn new() -> Self {
        Board::from_fn(|x, y| match y {
            0..=3 if (x + y) % 2 == 1 => Piece::BlackPawn,
            6..=9 if (x + y) % 2 == 1 => Piece::WhitePawn,
            _ => Piece::Empty,
        })
    }

    fn position_from_index(index: usize) -> Position {
        (index / MATRIX_SIZE, index % MATRIX_SIZE).into()
    }

    pub fn iter(&self) -> BoardIter<'_> {
        self.cells.iter()
    }

    // TODO
    // pub fn find_best_move(color: PieceColor) -> Move {}
}

pub fn filter_by<I, C, F>(iter: I, f: F) -> Vec<C>
where
    C: Cell,
    I: Iterator<Item = C>,
    F: Fn(&C) -> bool,
{
    iter.filter(f).collect()
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
        let pos: (usize, usize) = pos.into();
        self.cells.index(pos)
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
        let pos: (usize, usize) = pos.into();
        self.cells.index_mut(pos)
    }
}

#[test]
fn test_position_from_index() {
    assert_eq!(Board::position_from_index(9), Position((0, 9)));
    assert_eq!(Board::position_from_index(15), Position((1, 5)));
    assert_eq!(Board::position_from_index(10), Position((1, 0)));
    assert_eq!(Board::position_from_index(19), Position((1, 9)));
}
#[test]
fn test_indexing_usize() {
    let board = Board::new();
    assert_eq!(board[1], Piece::BlackPawn);
    assert_eq!(board[0], Piece::Empty);
    assert_eq!(board[98], Piece::WhitePawn);
}

#[test]
fn test_filter_white_pawns() {
    let board = Board::new();
    let iter = board.iter();
    let white_pawns = filter_by(iter.enumerate(), Cell::is_white);
    assert_eq!(white_pawns.len(), 20);
    assert_eq!(white_pawns[0], (61, &Piece::WhitePawn));
    assert_eq!(white_pawns[1], (63, &Piece::WhitePawn));
    assert_eq!(white_pawns[2], (65, &Piece::WhitePawn));
    assert_eq!(white_pawns.last(), Some(&(98, &Piece::WhitePawn)));
}

#[test]
fn test_filter_black_pawns() {
    let board = Board::new();
    let iter = board.iter();
    let white_pawns = filter_by(iter.enumerate(), Cell::is_black);
    assert_eq!(white_pawns.len(), 20);
    assert_eq!(white_pawns[0], (1, &Piece::BlackPawn));
    assert_eq!(white_pawns[1], (3, &Piece::BlackPawn));
    assert_eq!(white_pawns[2], (5, &Piece::BlackPawn));
    assert_eq!(white_pawns.last(), Some(&(38, &Piece::BlackPawn)));
}
