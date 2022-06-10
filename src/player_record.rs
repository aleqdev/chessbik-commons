use crate::{IsOwning, OpponentName};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerRecord {
    None,
    Engine(IsOwning),
    Opponent(OpponentName, IsOwning),
}
