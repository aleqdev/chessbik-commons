use chessbik_board::{PieceTy, PieceColor};
use serde::{Serialize, Serializer};

use super::*;

impl Serialize for Cell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer 
    {
        let first_char = match self.piece {
            Some(piece) => match (piece.ty, piece.color) {
                (PieceTy::PAWN, PieceColor::WHITE) => 'P',
                (PieceTy::PAWN, PieceColor::BLACK) => 'p',
                (PieceTy::ROOK, PieceColor::WHITE) => 'R',
                (PieceTy::ROOK, PieceColor::BLACK) => 'r',
                (PieceTy::KNIGHT, PieceColor::WHITE) => 'N',
                (PieceTy::KNIGHT, PieceColor::BLACK) => 'n',
                (PieceTy::BISHOP, PieceColor::WHITE) => 'B',
                (PieceTy::BISHOP, PieceColor::BLACK) => 'b',
                (PieceTy::QUEEN, PieceColor::WHITE) => 'Q',
                (PieceTy::QUEEN, PieceColor::BLACK) => 'q',
                (PieceTy::KING, PieceColor::WHITE) => 'K',
                (PieceTy::KING, PieceColor::BLACK) => 'k',
                (PieceTy::MAGE, PieceColor::WHITE) => 'M',
                (PieceTy::MAGE, PieceColor::BLACK) => 'm',
            },
            None => '-',
        };

        let second_char = self.side.into();
        
        serializer.serialize_str([first_char, second_char].into_iter().collect::<String>().as_str())
    }
}