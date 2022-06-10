use chessbik_board::Board;

use crate::{Lobby, Cell, PlayerColor, PlayerToken, PlayersRecord, OpponentName};

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
    RequestPlayers(Lobby, PlayerToken),
    RequestPlayesrCallback(PlayersRecord),
    RequestPlayerRemoval(Lobby, PlayerColor, PlayerToken),
    RequestPlayerNameUpdate(Lobby, PlayerColor, PlayerToken, OpponentName),
    JoinGame(Lobby),
    ConsiderRequestingBoard,
    ConsiderRequestingPlayers
}
