use chessbik_board::GetAvailableMoves;

use super::*;

impl GetAvailableMoves<Cell> for Cell {
    fn get_available_moves(
        &self,
        pos: impl Into<chessbik_board::PiecePosition>,
        board: &chessbik_board::Board<Cell>,
    ) -> Vec<chessbik_board::PieceMove> {
        match self.piece {
            Some(piece) => piece.get_available_moves(pos, board),
            None => vec!()
        }
    }
}
