use chessbik_board::{PiecePosition, PieceMove};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PieceMovePair {
    pub from: PiecePosition,
    pub mv: PieceMove
}
