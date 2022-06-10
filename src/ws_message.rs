use chessbik_board::Board;

use crate::{Lobby, Cell, PlayerColor, PlayerToken};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WsMessage {
    CreateGame,
    CreateGameCallback(Lobby),
    RequestPlayerToken,
    RequestPlayerTokenCallback(PlayerToken),
    RequestBoard(Lobby),
    RequestBoardCallback(Board<Cell>),
    RequestOpponentAddition(Lobby, PlayerColor, PlayerToken),
    RequestEngineAddition(Lobby, PlayerColor, PlayerToken),
    RequestPlayerOwning(Lobby, PlayerColor, PlayerToken),
    RequestPlayerOwningCallback(PlayerColor, bool),
    RequestPlayerRemoval(Lobby, PlayerColor, PlayerToken),
    JoinGame(Lobby)
}
