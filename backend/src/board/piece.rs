use std::fmt;

use crate::board::cell::Cell;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum Piece {
    WhitePawn = 1,
    BlackPawn = 2,
    WhiteQueen = 11,
    BlackQueen = 12,
    Empty = 0,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Variant {
    Queen,
    Pawn,
    None
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Color {
    Black,
    White,
    None,
}

impl Piece {
    /// promotes the pawn to queen
    /// ```
    /// let mut piece = Piece::BlackPawn;
    /// piece.promote();
    /// assert_eq!(piece, Piece::BlackQueen);
    /// ```
    pub fn promote(&mut self) {
        if self.is_empty() || self.is_queen() {
            return
        }

        *self = if self.is_white() { Piece::WhiteQueen } else { Piece::BlackQueen };
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Piece::*;
        let display_value = match self {
            Empty => "[--]",
            BlackPawn => "[BP]",
            BlackQueen => "[BQ]",
            WhitePawn => "[WP]",
            WhiteQueen => "[WQ]",
        };
        write!(f, "{}", display_value)
    }
}

impl Cell for Piece {
    type Variant = Variant;
    type Color = Color;
    type Enemy = Self;

    fn is_empty(&self) -> bool {
        *self == Piece::Empty
    }

    /// returns `true` if piece is white
    fn is_white(&self) -> bool {
        use Piece::{ WhiteQueen, WhitePawn };
        *self == WhitePawn || *self == WhiteQueen
    }

    /// returns `true` if piece is black
    fn is_black(&self) -> bool {
        use Piece::{ BlackQueen, BlackPawn };
        *self == BlackQueen || *self == BlackPawn
    }

    /// returns `true` if piece is a pawn
    fn is_pawn(&self) -> bool {
        use Piece::{ BlackPawn, WhitePawn };
        *self == BlackPawn || *self == WhitePawn
    }

    /// returns `true` if piece is a queen
    fn is_queen(&self) -> bool {
        use Piece::{ BlackQueen, WhiteQueen };
        *self == BlackQueen || *self == WhiteQueen
    }

    /// returns `true`, if `self` and `other` have different colors.
    fn is_enemy(&self, other: &Self::Enemy) -> bool {
        self.is_black() && other.is_white() || self.is_white() && other.is_black()
    }

    /// returns `PieceVariant` of the piece
    ///
    /// ```
    /// assert_eq!(Piece::Empty.piece_type(), PieceType::None);
    /// assert_eq!(Piece::WhitePawn.piece_type(), PieceType::Pawn);
    /// assert_eq!(Piece::BlackPawn.piece_type(), PieceType::Pawn);
    /// assert_eq!(Piece::BlackQueen.piece_type(), PieceType::Queen);
    /// assert_eq!(Piece::WhiteQueen.piece_type(), PieceType::Queen);
    /// ```
    fn variant(&self) -> Self::Variant {
        if self.is_pawn() {
            Self::Variant::Pawn
        } else if self.is_queen() {
            Self::Variant::Queen
        } else {
            Self::Variant::None
        }
    }

    /// returns `Color` of the piece
    fn color(&self) -> Self::Color {
        if self.is_white() {
            Self::Color::White
        } else if self.is_black() {
            Self::Color::Black
        } else {
            Self::Color::None
        }
    }
}


#[test]
fn is_empty_works() {
    assert!(!Piece::BlackPawn.is_empty());
    assert!(!Piece::WhitePawn.is_empty());
    assert!(!Piece::WhiteQueen.is_empty());
    assert!(!Piece::BlackQueen.is_empty());
    assert!(Piece::Empty.is_empty());
}

#[test]
fn is_white_works() {
    assert!(!Piece::Empty.is_white());
    assert!(!Piece::BlackQueen.is_white());
    assert!(!Piece::BlackPawn.is_white());
    assert!(Piece::WhiteQueen.is_white());
    assert!(Piece::WhitePawn.is_white());
}

#[test]
fn is_black_works() {
    assert!(!Piece::Empty.is_black());
    assert!(!Piece::WhitePawn.is_black());
    assert!(!Piece::WhiteQueen.is_black());
    assert!(Piece::BlackPawn.is_black());
    assert!(Piece::BlackQueen.is_black());
}

#[test]
fn is_pawn_works() {
    assert!(!Piece::Empty.is_pawn());
    assert!(!Piece::WhiteQueen.is_pawn());
    assert!(!Piece::BlackQueen.is_pawn());
    assert!(Piece::BlackPawn.is_pawn());
    assert!(Piece::WhitePawn.is_pawn());
}

#[test]
fn is_queen_works() {
    assert!(!Piece::Empty.is_queen());
    assert!(!Piece::BlackPawn.is_queen());
    assert!(!Piece::WhitePawn.is_queen());
    assert!(Piece::WhiteQueen.is_queen());
    assert!(Piece::BlackQueen.is_queen());
}

#[test]
fn is_enemy_works() {
    assert!(Piece::WhitePawn.is_enemy(&Piece::BlackPawn));
    assert!(Piece::WhitePawn.is_enemy(&Piece::BlackQueen));
    assert!(Piece::WhiteQueen.is_enemy(&Piece::BlackPawn));
    assert!(Piece::WhiteQueen.is_enemy(&Piece::BlackQueen));
    assert!(!Piece::BlackPawn.is_enemy(&Piece::BlackQueen));
    assert!(!Piece::BlackPawn.is_enemy(&Piece::BlackPawn));
}

#[test]
fn promote_works() {
    let mut piece = Piece::BlackPawn;
    piece.promote();
    assert_eq!(piece, Piece::BlackQueen);

    let mut piece = Piece::WhitePawn;
    piece.promote();
    assert_eq!(piece, Piece::WhiteQueen);
}

#[test]
fn variant_works() {
    assert_eq!(Piece::Empty.variant(), Variant::None);
    assert_eq!(Piece::WhitePawn.variant(), Variant::Pawn);
    assert_eq!(Piece::BlackPawn.variant(), Variant::Pawn);
    assert_eq!(Piece::BlackQueen.variant(), Variant::Queen);
    assert_eq!(Piece::WhiteQueen.variant(), Variant::Queen);
}

#[test]
fn color_works() {
    assert_eq!(Piece::Empty.color(), Color::None);
    assert_eq!(Piece::WhitePawn.color(), Color::White);
    assert_eq!(Piece::BlackPawn.color(), Color::Black);
    assert_eq!(Piece::BlackQueen.color(), Color::Black);
    assert_eq!(Piece::WhiteQueen.color(), Color::White);
}
