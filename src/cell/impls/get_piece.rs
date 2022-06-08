use chessbik_board::GetPiece;

use super::*;

impl GetPiece for Cell {
    fn get_piece(&self) -> Option<&Piece> {
        self.piece.as_ref()
    }
}