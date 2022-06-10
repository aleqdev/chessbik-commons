use crate::PlayerToken;

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
