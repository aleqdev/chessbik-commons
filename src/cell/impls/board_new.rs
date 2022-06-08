use chessbik_board::{BoardNew, Board};

use super::*;

impl BoardNew for Cell {
    fn board_new() -> Board<Self> {
        Board { 
            cells: [
                Cell::black_pawn(Side::TOP),
                Cell::black_pawn(Side::TOP),
                Cell::black_pawn(Side::TOP),
                Cell::none(Side::TOP),
                Cell::none(Side::TOP),
                Cell::none(Side::TOP),
                Cell::white_pawn(Side::TOP),
                Cell::white_pawn(Side::TOP),
                Cell::white_pawn(Side::TOP),

                Cell::black_pawn(Side::LEFT),
                Cell::none(Side::LEFT),
                Cell::white_pawn(Side::LEFT),
                Cell::black_pawn(Side::LEFT),
                Cell::none(Side::LEFT),
                Cell::white_pawn(Side::LEFT),
                Cell::black_pawn(Side::LEFT),
                Cell::none(Side::LEFT),
                Cell::white_pawn(Side::LEFT),

                Cell::white_knight(Side::FORWARD),
                Cell::white_queen(Side::FORWARD),
                Cell::white_knight(Side::FORWARD),
                Cell::white_rook(Side::FORWARD),
                Cell::white_king(Side::FORWARD),
                Cell::white_rook(Side::FORWARD),
                Cell::white_bishop(Side::FORWARD),
                Cell::white_mage(Side::FORWARD),
                Cell::white_bishop(Side::FORWARD),

                Cell::white_pawn(Side::RIGHT),
                Cell::none(Side::RIGHT),
                Cell::black_pawn(Side::RIGHT),
                Cell::white_pawn(Side::RIGHT),
                Cell::none(Side::RIGHT),
                Cell::black_pawn(Side::RIGHT),
                Cell::white_pawn(Side::RIGHT),
                Cell::none(Side::RIGHT),
                Cell::black_pawn(Side::RIGHT),

                Cell::black_knight(Side::BACK),
                Cell::black_queen(Side::BACK),
                Cell::black_knight(Side::BACK),
                Cell::black_rook(Side::BACK),
                Cell::black_king(Side::BACK),
                Cell::black_rook(Side::BACK),
                Cell::black_bishop(Side::BACK),
                Cell::black_mage(Side::BACK),
                Cell::black_bishop(Side::BACK),

                Cell::white_pawn(Side::BOTTOM),
                Cell::white_pawn(Side::BOTTOM),
                Cell::white_pawn(Side::BOTTOM),
                Cell::none(Side::BOTTOM),
                Cell::none(Side::BOTTOM),
                Cell::none(Side::BOTTOM),
                Cell::black_pawn(Side::BOTTOM),
                Cell::black_pawn(Side::BOTTOM),
                Cell::black_pawn(Side::BOTTOM),
            ]
        }
    }
}