use chessbik_board::PieceColor;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerColor {
    WHITE,
    BLACK
}

impl PlayerColor {
    pub fn eq_piece_color(&self, c: PieceColor) -> bool {
        match self {
            PlayerColor::WHITE => c == PieceColor::WHITE,
            PlayerColor::BLACK => c == PieceColor::BLACK,
        }
    }
}