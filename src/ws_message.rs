use chessbik_board::Board;

use crate::{Lobby, Cell};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WsMessage {
    CreateGame,
    CreateGameCallback(Lobby),
    RequestBoard(Lobby),
    RequestBoardCallback(Board<Cell>)
}
