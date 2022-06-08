chessbik_derive_wrapper::derive_wrapper!(
    #[derive(
        Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
    )]
    pub struct Lobby(String);
);

impl Lobby {
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        assert!(s.len() == 7, "lobby string must be 7 characters");
        Self(s)
    }
}
