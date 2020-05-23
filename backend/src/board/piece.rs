use std::fmt;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum Piece {
    WhitePawn = 1,
    BlackPawn = 2,
    WhiteQueen = 11,
    BlackQueen = 12,
    Empty = 0,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PieceType {
    Queen,
    Pawn,
    None
}

impl Piece {
    pub fn is_empty(&self) -> bool {
        *self == Piece::Empty
    }

    /// returns true if piece is white
    pub fn is_white(&self) -> bool {
        use Piece::{ WhiteQueen, WhitePawn };
        *self == WhitePawn || *self == WhiteQueen
    }

    /// returns true if piece is black
    pub fn is_black(&self) -> bool {
        use Piece::{ BlackQueen, BlackPawn };
        *self == BlackQueen || *self == BlackPawn
    }

    /// returns `true` if piece is a pawn
    pub fn is_pawn(&self) -> bool {
        use Piece::{ BlackPawn, WhitePawn };
        *self == BlackPawn || *self == WhitePawn
    }

    /// returns `true` if piece is a queen
    pub fn is_queen(&self) -> bool {
        use Piece::{ BlackQueen, WhiteQueen };
        *self == BlackQueen || *self == WhiteQueen
    }

    /// returns `PieceType` of the piece
    ///
    /// ```
    /// assert_eq!(Piece::Empty.piece_type(), PieceType::None);
    /// assert_eq!(Piece::WhitePawn.piece_type(), PieceType::Pawn);
    /// assert_eq!(Piece::BlackPawn.piece_type(), PieceType::Pawn);
    /// assert_eq!(Piece::BlackQueen.piece_type(), PieceType::Queen);
    /// assert_eq!(Piece::WhiteQueen.piece_type(), PieceType::Queen);
    /// ```
    pub fn piece_type(&self) -> PieceType {
        if self.is_pawn() {
            PieceType::Pawn
        } else if self.is_queen() {
            PieceType::Queen
        } else {
            PieceType::None
        }
    }

    /// returns true, if `self` and `other` have different colors.
    pub fn is_enemy(&self, other: &Self) -> bool {
        self.is_black() && other.is_white() || self.is_white() && other.is_black()
    }

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
fn piece_type_works() {
    assert_eq!(Piece::Empty.piece_type(), PieceType::None);
    assert_eq!(Piece::WhitePawn.piece_type(), PieceType::Pawn);
    assert_eq!(Piece::BlackPawn.piece_type(), PieceType::Pawn);
    assert_eq!(Piece::BlackQueen.piece_type(), PieceType::Queen);
    assert_eq!(Piece::WhiteQueen.piece_type(), PieceType::Queen);
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
