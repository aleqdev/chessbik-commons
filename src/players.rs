use crate::{Player, PlayerColor};

pub struct Players {
    pub white: Player,
    pub black: Player
}

impl Players {
    pub fn playing(&mut self, color: PlayerColor) -> &mut Player {
        match color {
            PlayerColor::WHITE => &mut self.white,
            PlayerColor::BLACK => &mut self.black,
        }
    }
}