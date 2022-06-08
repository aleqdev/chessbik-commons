use crate::Lobby;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WsMessage {
    CreateGame,
    CreateGameCallback(Lobby),
}
