use crate::Lobby;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum WsMessage {
    CreateGame,
    CreateGameCallback(Lobby),
}
