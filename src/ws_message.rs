use chessbik_board::Board;

use crate::{Lobby, Cell, PlayerColor, PlayerToken, PlayersRecord, OpponentName};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WsMessage {
    CreateGame,
    RequestPlayerToken,
    RequestPlayerTokenCallback(PlayerToken),
    RequestBoard(Lobby),
    RequestBoardCallback(Board<Cell>),
    RequestOpponentAddition(Lobby, PlayerColor, PlayerToken, OpponentName),
    RequestEngineAddition(Lobby, PlayerColor, PlayerToken),
    RequestPlayers(Lobby, PlayerToken),
    RequestPlayersCallback(PlayersRecord),
    RequestPlayerRemoval(Lobby, PlayerColor, PlayerToken),
    RequestPlayerNameUpdate(Lobby, PlayerColor, PlayerToken, OpponentName),
    RequestGameSubscription(Lobby, PlayerToken),
    ConsiderRequestingBoard,
    ConsiderRequestingPlayers,
    ConsiderSubscription(Lobby)
}
