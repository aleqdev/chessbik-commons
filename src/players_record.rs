use crate::PlayerRecord;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayersRecord {
    pub white: PlayerRecord,
    pub black: PlayerRecord,
}
