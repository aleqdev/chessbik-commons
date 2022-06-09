pub use super::*;

pub mod display;
pub use display::*;

pub mod board_new;
pub use board_new::*;

pub mod get_piece;
pub use get_piece::*;

pub mod get_available_moves;
pub use get_available_moves::*;

pub mod serialize;
pub use serialize::*;

pub mod deserialize;
pub use deserialize::*;

impl Cell {
    pub fn some(piece: impl Into<Piece>, side: Side) -> Self {
        Self {
            piece: Some(piece.into()),
            side,
        }
    }

    pub fn none(side: Side) -> Self {
        Self { piece: None, side }
    }

    pub fn white_pawn(side: Side) -> Cell {
        Cell::some(Piece::WHITE_PAWN, side)
    }

    pub fn black_pawn(side: Side) -> Self {
        Self::some(Piece::BLACK_PAWN, side)
    }

    pub fn white_knight(side: Side) -> Self {
        Self::some(Piece::WHITE_KNIGHT, side)
    }

    pub fn black_knight(side: Side) -> Self {
        Self::some(Piece::BLACK_KNIGHT, side)
    }

    pub fn white_bishop(side: Side) -> Self {
        Self::some(Piece::WHITE_BISHOP, side)
    }

    pub fn black_bishop(side: Side) -> Self {
        Self::some(Piece::BLACK_BISHOP, side)
    }

    pub fn white_rook(side: Side) -> Self {
        Self::some(Piece::WHITE_ROOK, side)
    }

    pub fn black_rook(side: Side) -> Self {
        Self::some(Piece::BLACK_ROOK, side)
    }

    pub fn white_queen(side: Side) -> Self {
        Self::some(Piece::WHITE_QUEEN, side)
    }

    pub fn black_queen(side: Side) -> Self {
        Self::some(Piece::BLACK_QUEEN, side)
    }

    pub fn white_king(side: Side) -> Self {
        Self::some(Piece::WHITE_KING, side)
    }

    pub fn black_king(side: Side) -> Self {
        Self::some(Piece::BLACK_KING, side)
    }

    pub fn white_mage(side: Side) -> Self {
        Self::some(Piece::WHITE_MAGE, side)
    }

    pub fn black_mage(side: Side) -> Self {
        Self::some(Piece::BLACK_MAGE, side)
    }
}
