use crate::PlayerToken;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player {
    None,
    Engine(PlayerToken),
    Opponent(PlayerToken),
}

impl Default for Player {
    fn default() -> Self {
        Self::None
    }
}
