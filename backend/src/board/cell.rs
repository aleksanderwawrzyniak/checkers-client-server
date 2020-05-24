use crate::board::piece::{Piece, Variant, Color};

pub trait Cell {
    type Variant;
    type Color;
    type Enemy;

    fn is_empty(&self) -> bool;
    fn is_white(&self) -> bool;
    fn is_black(&self) -> bool;
    fn is_pawn(&self) -> bool;
    fn is_queen(&self) -> bool;
    fn is_enemy(&self, other: &Self::Enemy) -> bool;
    fn variant(&self) -> Self::Variant;
    fn color(&self) -> Self::Color;
}

impl Cell for (usize, &Piece) {
    type Variant = Variant;
    type Color = Color;
    type Enemy = Piece;

    fn is_empty(&self) -> bool {
        *self.1 == Piece::Empty
    }

    /// returns `true` if piece is white
    fn is_white(&self) -> bool {
        use Piece::{ WhiteQueen, WhitePawn };
        *self.1 == WhitePawn || *self.1 == WhiteQueen
    }

    /// returns `true` if piece is black
    fn is_black(&self) -> bool {
        use Piece::{ BlackQueen, BlackPawn };
        *self.1 == BlackQueen || *self.1 == BlackPawn
    }

    /// returns `true` if piece is a pawn
    fn is_pawn(&self) -> bool {
        use Piece::{ BlackPawn, WhitePawn };
        *self.1 == BlackPawn || *self.1 == WhitePawn
    }

    /// returns `true` if piece is a queen
    fn is_queen(&self) -> bool {
        use Piece::{ BlackQueen, WhiteQueen };
        *self.1 == BlackQueen || *self.1 == WhiteQueen
    }

    /// returns `true`, if `self` and `other` have different colors.
    fn is_enemy(&self, other: &Self::Enemy) -> bool {
        self.1.is_black() && other.is_white() || self.1.is_white() && other.is_black()
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
        if self.1.is_pawn() {
            Self::Variant::Pawn
        } else if self.1.is_queen() {
            Self::Variant::Queen
        } else {
            Self::Variant::None
        }
    }

    /// returns `Color` of the piece
    fn color(&self) -> Self::Color {
        if self.1.is_white() {
            Self::Color::White
        } else if self.1.is_black() {
            Self::Color::Black
        } else {
            Self::Color::None
        }
    }
}

impl Cell for (usize, Piece) {
    type Variant = Variant;
    type Color = Color;
    type Enemy = Piece;

    fn is_empty(&self) -> bool {
        self.1 == Piece::Empty
    }

    /// returns `true` if piece is white
    fn is_white(&self) -> bool {
        use Piece::{ WhiteQueen, WhitePawn };
        self.1 == WhitePawn || self.1 == WhiteQueen
    }

    /// returns `true` if piece is black
    fn is_black(&self) -> bool {
        use Piece::{ BlackQueen, BlackPawn };
        self.1 == BlackQueen || self.1 == BlackPawn
    }

    /// returns `true` if piece is a pawn
    fn is_pawn(&self) -> bool {
        use Piece::{ BlackPawn, WhitePawn };
        self.1 == BlackPawn || self.1 == WhitePawn
    }

    /// returns `true` if piece is a queen
    fn is_queen(&self) -> bool {
        use Piece::{ BlackQueen, WhiteQueen };
        self.1 == BlackQueen || self.1 == WhiteQueen
    }

    /// returns `true`, if `self` and `other` have different colors.
    fn is_enemy(&self, other: &Self::Enemy) -> bool {
        self.1.is_black() && other.is_white() || self.1.is_white() && other.is_black()
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
        if self.1.is_pawn() {
            Self::Variant::Pawn
        } else if self.1.is_queen() {
            Self::Variant::Queen
        } else {
            Self::Variant::None
        }
    }

    /// returns `Color` of the piece
    fn color(&self) -> Self::Color {
        if self.1.is_white() {
            Self::Color::White
        } else if self.1.is_black() {
            Self::Color::Black
        } else {
            Self::Color::None
        }
    }
}

